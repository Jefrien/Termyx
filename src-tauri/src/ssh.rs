use std::{
    net::{TcpStream, ToSocketAddrs},
    path::Path,
    time::Duration,
};

use serde::Serialize;
use ssh2::Session;
use tauri::AppHandle;

use crate::vault::{get_vault_host_with_credential, CredentialRecord, HostRecord};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SshConnectionResult {
    status: String,
    message: String,
}

fn result(status: &str, message: impl Into<String>) -> SshConnectionResult {
    SshConnectionResult {
        status: status.to_string(),
        message: message.into(),
    }
}

fn connect_tcp(host: &HostRecord, timeout: Duration) -> Result<TcpStream, SshConnectionResult> {
    let address = format!("{}:{}", host.hostname, host.port);
    let socket_address = address
        .to_socket_addrs()
        .map_err(|error| result("networkFailed", format!("Failed to resolve host: {error}")))?
        .next()
        .ok_or_else(|| result("networkFailed", "Host did not resolve to an address"))?;
    let stream = TcpStream::connect_timeout(&socket_address, timeout)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::TimedOut {
                result("timeout", "SSH connection timed out")
            } else {
                result("networkFailed", format!("Failed to connect: {error}"))
            }
        })?;

    stream
        .set_read_timeout(Some(timeout))
        .map_err(|error| result("networkFailed", format!("Failed to set read timeout: {error}")))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|error| result("networkFailed", format!("Failed to set write timeout: {error}")))?;

    Ok(stream)
}

fn authenticate(
    session: &mut Session,
    host: &HostRecord,
    credential: Option<CredentialRecord>,
) -> Result<(), SshConnectionResult> {
    let credential = credential.ok_or_else(|| {
        result(
            "missingCredentials",
            "No saved credential found for this host",
        )
    })?;

    match credential.auth_type.as_str() {
        "password" => {
            let password = credential
                .password
                .ok_or_else(|| result("missingCredentials", "Missing saved password"))?;

            session
                .userauth_password(&host.username, &password)
                .map_err(|_| result("authFailed", "SSH password authentication failed"))
        }
        "privateKey" => {
            if credential.private_key_storage_mode.as_deref() == Some("imported") {
                let private_key = credential
                    .private_key_content
                    .ok_or_else(|| result("missingCredentials", "Missing imported private key"))?;

                session
                    .userauth_pubkey_memory(&host.username, None, &private_key, None)
                    .map_err(|_| result("authFailed", "SSH private key authentication failed"))
            } else {
                let private_key_path = credential
                    .private_key_path
                    .ok_or_else(|| result("missingCredentials", "Missing private key path"))?;

                session
                    .userauth_pubkey_file(&host.username, None, Path::new(&private_key_path), None)
                    .map_err(|_| result("authFailed", "SSH private key authentication failed"))
            }
        }
        _ => Err(result(
            "missingCredentials",
            "Unsupported or missing authentication method",
        )),
    }
}

fn test_connection_blocking(
    app: AppHandle,
    passphrase: String,
    host_id: String,
    timeout_ms: u64,
) -> SshConnectionResult {
    let timeout = Duration::from_millis(timeout_ms.clamp(1_000, 30_000));
    let (host, credential) =
        match get_vault_host_with_credential(&app, &passphrase, &host_id) {
            Ok(value) => value,
            Err(error) => return result("vaultError", error),
        };
    let tcp_stream = match connect_tcp(&host, timeout) {
        Ok(stream) => stream,
        Err(error) => return error,
    };
    let mut session = match Session::new() {
        Ok(session) => session,
        Err(error) => return result("networkFailed", format!("Failed to create SSH session: {error}")),
    };

    session.set_tcp_stream(tcp_stream);
    session.set_timeout(timeout.as_millis() as u32);

    if let Err(error) = session.handshake() {
        return result("networkFailed", format!("SSH handshake failed: {error}"));
    }

    if let Err(error) = authenticate(&mut session, &host, credential) {
        return error;
    }

    if session.authenticated() {
        result("connected", "SSH authentication succeeded")
    } else {
        result("authFailed", "SSH authentication failed")
    }
}

#[tauri::command]
pub async fn test_ssh_connection(
    app: AppHandle,
    passphrase: String,
    host_id: String,
    timeout_ms: u64,
) -> Result<SshConnectionResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        test_connection_blocking(app, passphrase, host_id, timeout_ms)
    })
    .await
    .map_err(|error| format!("SSH worker failed: {error}"))
}
