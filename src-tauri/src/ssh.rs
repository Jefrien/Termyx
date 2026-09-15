use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    path::Path,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use serde::Serialize;
use ssh2::{Channel, Session};
use tauri::{AppHandle, Emitter, State};

use crate::vault::{get_vault_host_with_credential, CredentialRecord, HostRecord};

#[derive(Default)]
pub struct SshSessionRegistry {
    sessions: Arc<Mutex<HashMap<String, Sender<SshWorkerCommand>>>>,
}

#[derive(Debug)]
enum SshWorkerCommand {
    Resize { cols: u32, rows: u32 },
    Stop,
    Write(String),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SshConnectionResult {
    status: String,
    message: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SshSessionDataEvent {
    session_id: String,
    data: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SshSessionStatusEvent {
    session_id: String,
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
    let stream = TcpStream::connect_timeout(&socket_address, timeout).map_err(|error| {
        if error.kind() == std::io::ErrorKind::TimedOut {
            result("timeout", "SSH connection timed out")
        } else {
            result("networkFailed", format!("Failed to connect: {error}"))
        }
    })?;

    stream
        .set_read_timeout(Some(Duration::from_millis(200)))
        .map_err(|error| result("networkFailed", format!("Failed to set read timeout: {error}")))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|error| {
            result(
                "networkFailed",
                format!("Failed to set write timeout: {error}"),
            )
        })?;

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

fn emit_data(app: &AppHandle, session_id: &str, data: impl Into<String>) {
    let _ = app.emit(
        "ssh-session-data",
        SshSessionDataEvent {
            session_id: session_id.to_string(),
            data: data.into(),
        },
    );
}

fn emit_status(app: &AppHandle, session_id: &str, status: &str, message: impl Into<String>) {
    let _ = app.emit(
        "ssh-session-status",
        SshSessionStatusEvent {
            session_id: session_id.to_string(),
            status: status.to_string(),
            message: message.into(),
        },
    );
}

fn open_shell_channel(
    session: &Session,
    cols: u32,
    rows: u32,
) -> Result<Channel, SshConnectionResult> {
    let mut channel = session
        .channel_session()
        .map_err(|error| result("networkFailed", format!("Failed to open SSH channel: {error}")))?;

    channel
        .request_pty("xterm-256color", None, Some((cols, rows, 0, 0)))
        .map_err(|error| result("networkFailed", format!("Failed to request PTY: {error}")))?;
    channel
        .shell()
        .map_err(|error| result("networkFailed", format!("Failed to start remote shell: {error}")))?;

    Ok(channel)
}

fn handle_worker_command(channel: &mut Channel, command: SshWorkerCommand) -> bool {
    match command {
        SshWorkerCommand::Resize { cols, rows } => {
            let _ = channel.request_pty_size(cols, rows, None, None);
            true
        }
        SshWorkerCommand::Stop => false,
        SshWorkerCommand::Write(data) => {
            let _ = channel.write_all(data.as_bytes());
            let _ = channel.flush();
            true
        }
    }
}

fn run_channel_loop(
    app: AppHandle,
    registry: Arc<Mutex<HashMap<String, Sender<SshWorkerCommand>>>>,
    session_id: String,
    mut channel: Channel,
    receiver: Receiver<SshWorkerCommand>,
) {
    let mut buffer = [0_u8; 8192];

    loop {
        while let Ok(command) = receiver.try_recv() {
            if !handle_worker_command(&mut channel, command) {
                let _ = channel.close();
                let _ = registry.lock().map(|mut sessions| sessions.remove(&session_id));
                emit_status(&app, &session_id, "disconnected", "SSH session disconnected");
                return;
            }
        }

        match channel.read(&mut buffer) {
            Ok(0) => {
                if channel.eof() {
                    let _ = registry.lock().map(|mut sessions| sessions.remove(&session_id));
                    emit_status(&app, &session_id, "disconnected", "SSH session closed");
                    return;
                }
            }
            Ok(size) => {
                emit_data(&app, &session_id, String::from_utf8_lossy(&buffer[..size]).to_string());
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(error) if error.kind() == std::io::ErrorKind::TimedOut => {}
            Err(error) => {
                let _ = registry.lock().map(|mut sessions| sessions.remove(&session_id));
                emit_status(&app, &session_id, "networkFailed", format!("SSH read failed: {error}"));
                return;
            }
        }

        thread::sleep(Duration::from_millis(12));
    }
}

fn start_session_blocking(
    app: AppHandle,
    registry: Arc<Mutex<HashMap<String, Sender<SshWorkerCommand>>>>,
    ready_sender: Sender<SshConnectionResult>,
    passphrase: String,
    host_id: String,
    session_id: String,
    cols: u32,
    rows: u32,
    timeout_ms: u64,
) {
    let timeout = Duration::from_millis(timeout_ms.clamp(1_000, 30_000));
    let (host, credential) =
        match get_vault_host_with_credential(&app, &passphrase, &host_id) {
            Ok(value) => value,
            Err(error) => {
                let _ = ready_sender.send(result("vaultError", error));
                return;
            }
        };
    let tcp_stream = match connect_tcp(&host, timeout) {
        Ok(stream) => stream,
        Err(error) => {
            let _ = ready_sender.send(error);
            return;
        }
    };
    let mut session = match Session::new() {
        Ok(session) => session,
        Err(error) => {
            let _ = ready_sender.send(result(
                "networkFailed",
                format!("Failed to create SSH session: {error}"),
            ));
            return;
        }
    };

    session.set_tcp_stream(tcp_stream);
    session.set_timeout(timeout.as_millis() as u32);

    if let Err(error) = session.handshake() {
        let _ = ready_sender.send(result("networkFailed", format!("SSH handshake failed: {error}")));
        return;
    }

    if let Err(error) = authenticate(&mut session, &host, credential) {
        let _ = ready_sender.send(error);
        return;
    }

    if !session.authenticated() {
        let _ = ready_sender.send(result("authFailed", "SSH authentication failed"));
        return;
    }

    session.set_blocking(false);

    let channel = match open_shell_channel(&session, cols, rows) {
        Ok(channel) => channel,
        Err(error) => {
            let _ = ready_sender.send(error);
            return;
        }
    };
    let (sender, receiver) = mpsc::channel();

    if let Ok(mut sessions) = registry.lock() {
        sessions.insert(session_id.clone(), sender);
    }

    emit_status(&app, &session_id, "connected", "SSH shell started");
    emit_data(&app, &session_id, "\r\nSSH shell started.\r\n");
    let _ = ready_sender.send(result("connected", "SSH shell started"));
    run_channel_loop(app, registry, session_id, channel, receiver);
}

#[tauri::command]
pub async fn start_ssh_session(
    app: AppHandle,
    registry: State<'_, SshSessionRegistry>,
    passphrase: String,
    host_id: String,
    session_id: String,
    cols: u32,
    rows: u32,
    timeout_ms: u64,
) -> Result<SshConnectionResult, String> {
    let registry = registry.sessions.clone();
    let (ready_sender, ready_receiver) = mpsc::channel();

    thread::spawn(move || {
        start_session_blocking(
            app,
            registry,
            ready_sender,
            passphrase,
            host_id,
            session_id,
            cols.max(20),
            rows.max(6),
            timeout_ms,
        );
    });

    ready_receiver
        .recv_timeout(Duration::from_millis(timeout_ms.clamp(1_000, 30_000) + 1_000))
        .map_err(|_| String::from("SSH worker did not report readiness"))
}

#[tauri::command]
pub fn write_ssh_session(
    registry: State<'_, SshSessionRegistry>,
    session_id: String,
    data: String,
) -> Result<(), String> {
    let sessions = registry
        .sessions
        .lock()
        .map_err(|_| String::from("SSH session registry is unavailable"))?;
    let sender = sessions
        .get(&session_id)
        .ok_or_else(|| String::from("SSH session not found"))?;

    sender
        .send(SshWorkerCommand::Write(data))
        .map_err(|error| format!("Failed to write to SSH session: {error}"))
}

#[tauri::command]
pub fn resize_ssh_session(
    registry: State<'_, SshSessionRegistry>,
    session_id: String,
    cols: u32,
    rows: u32,
) -> Result<(), String> {
    let sessions = registry
        .sessions
        .lock()
        .map_err(|_| String::from("SSH session registry is unavailable"))?;
    let sender = sessions
        .get(&session_id)
        .ok_or_else(|| String::from("SSH session not found"))?;

    sender
        .send(SshWorkerCommand::Resize { cols, rows })
        .map_err(|error| format!("Failed to resize SSH session: {error}"))
}

#[tauri::command]
pub fn stop_ssh_session(
    registry: State<'_, SshSessionRegistry>,
    session_id: String,
) -> Result<(), String> {
    let sessions = registry
        .sessions
        .lock()
        .map_err(|_| String::from("SSH session registry is unavailable"))?;
    let sender = sessions
        .get(&session_id)
        .ok_or_else(|| String::from("SSH session not found"))?;

    sender
        .send(SshWorkerCommand::Stop)
        .map_err(|error| format!("Failed to stop SSH session: {error}"))
}
