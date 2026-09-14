use std::{fs, path::PathBuf};

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Algorithm, Argon2, Params, Version,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

const VAULT_FILE_NAME: &str = "termyx-vault.json.enc";
const VAULT_VERSION: u16 = 1;
const KDF_NAME: &str = "argon2id";
const CIPHER_NAME: &str = "aes-256-gcm";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostRecord {
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub username: String,
    pub port: u16,
    pub favorite: bool,
    pub status: String,
    #[serde(rename = "authMethod")]
    #[serde(default = "default_auth_method")]
    pub auth_method: String,
    #[serde(rename = "privateKeyStorageMode")]
    #[serde(default)]
    pub private_key_storage_mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppVault {
    pub version: u16,
    pub hosts: Vec<HostRecord>,
    #[serde(default)]
    pub credentials: Vec<CredentialRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialRecord {
    pub host_id: String,
    pub auth_type: String,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub private_key_path: Option<String>,
    #[serde(default)]
    pub private_key_content: Option<String>,
    #[serde(default)]
    pub private_key_storage_mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EncryptedVaultFile {
    version: u16,
    kdf: String,
    cipher: String,
    salt: String,
    nonce: String,
    ciphertext: String,
}

fn default_auth_method() -> String {
    String::from("none")
}

fn vault_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;

    Ok(app_data_dir.join(VAULT_FILE_NAME))
}

fn derive_key(passphrase: &str, salt: &[u8]) -> Result<[u8; 32], String> {
    let salt = SaltString::encode_b64(salt)
        .map_err(|error| format!("Failed to encode vault salt: {error}"))?;
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(19_456, 2, 1, Some(32))
            .map_err(|error| format!("Failed to configure key derivation: {error}"))?,
    );
    let password_hash = argon2
        .hash_password(passphrase.as_bytes(), &salt)
        .map_err(|error| format!("Failed to derive vault key: {error}"))?;
    let hash = password_hash
        .hash
        .ok_or_else(|| String::from("Key derivation did not return a hash"))?;
    let hash_bytes = hash.as_bytes();
    let mut key = [0_u8; 32];

    key.copy_from_slice(hash_bytes);

    Ok(key)
}

fn encrypt_vault(passphrase: &str, vault: &AppVault) -> Result<EncryptedVaultFile, String> {
    let mut salt = [0_u8; 16];
    let mut nonce = [0_u8; 12];

    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let key = derive_key(passphrase, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|error| format!("Failed to create vault cipher: {error}"))?;
    let plaintext = serde_json::to_vec(vault)
        .map_err(|error| format!("Failed to serialize vault: {error}"))?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|error| format!("Failed to encrypt vault: {error}"))?;

    Ok(EncryptedVaultFile {
        version: VAULT_VERSION,
        kdf: KDF_NAME.to_string(),
        cipher: CIPHER_NAME.to_string(),
        salt: BASE64.encode(salt),
        nonce: BASE64.encode(nonce),
        ciphertext: BASE64.encode(ciphertext),
    })
}

fn decrypt_vault(passphrase: &str, encrypted: &EncryptedVaultFile) -> Result<AppVault, String> {
    if encrypted.version != VAULT_VERSION {
        return Err(format!("Unsupported vault version: {}", encrypted.version));
    }

    if encrypted.kdf != KDF_NAME || encrypted.cipher != CIPHER_NAME {
        return Err(String::from("Unsupported vault encryption format"));
    }

    let salt = BASE64
        .decode(&encrypted.salt)
        .map_err(|error| format!("Failed to decode vault salt: {error}"))?;
    let nonce = BASE64
        .decode(&encrypted.nonce)
        .map_err(|error| format!("Failed to decode vault nonce: {error}"))?;
    let ciphertext = BASE64
        .decode(&encrypted.ciphertext)
        .map_err(|error| format!("Failed to decode vault ciphertext: {error}"))?;
    let key = derive_key(passphrase, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|error| format!("Failed to create vault cipher: {error}"))?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| String::from("Failed to decrypt vault"))?;

    serde_json::from_slice(&plaintext)
        .map_err(|error| format!("Failed to deserialize vault: {error}"))
}

fn read_vault_file(app: &AppHandle, passphrase: &str) -> Result<Option<AppVault>, String> {
    let path = vault_path(app)?;

    if !path.exists() {
        return Ok(None);
    }

    let encrypted_bytes = fs::read(path)
        .map_err(|error| format!("Failed to read vault file: {error}"))?;
    let encrypted: EncryptedVaultFile = serde_json::from_slice(&encrypted_bytes)
        .map_err(|error| format!("Failed to parse vault file: {error}"))?;

    decrypt_vault(passphrase, &encrypted).map(Some)
}

fn write_vault_file(app: &AppHandle, passphrase: &str, vault: &AppVault) -> Result<(), String> {
    let path = vault_path(app)?;
    let parent = path
        .parent()
        .ok_or_else(|| String::from("Vault path does not have a parent directory"))?;
    let encrypted = encrypt_vault(passphrase, vault)?;
    let encrypted_bytes = serde_json::to_vec_pretty(&encrypted)
        .map_err(|error| format!("Failed to serialize encrypted vault: {error}"))?;

    fs::create_dir_all(parent)
        .map_err(|error| format!("Failed to create vault directory: {error}"))?;
    fs::write(path, encrypted_bytes)
        .map_err(|error| format!("Failed to write vault file: {error}"))
}

#[tauri::command]
pub fn vault_exists(app: AppHandle) -> Result<bool, String> {
    Ok(vault_path(&app)?.exists())
}

#[tauri::command]
pub fn reset_app_vault(app: AppHandle) -> Result<(), String> {
    let path = vault_path(&app)?;

    if path.exists() {
        fs::remove_file(path)
            .map_err(|error| format!("Failed to remove vault file: {error}"))?;
    }

    Ok(())
}

#[tauri::command]
pub fn load_app_vault(app: AppHandle, passphrase: String) -> Result<Option<AppVault>, String> {
    read_vault_file(&app, &passphrase)
}

#[tauri::command]
pub fn save_app_vault(
    app: AppHandle,
    passphrase: String,
    mut vault: AppVault,
) -> Result<(), String> {
    if let Some(existing_vault) = read_vault_file(&app, &passphrase)? {
        vault.credentials = existing_vault.credentials;
    }

    write_vault_file(&app, &passphrase, &vault)
}

#[tauri::command]
pub fn upsert_vault_host(
    app: AppHandle,
    passphrase: String,
    host: HostRecord,
    password: Option<String>,
    private_key_path: Option<String>,
    private_key_content: Option<String>,
    private_key_storage_mode: Option<String>,
    credential_action: String,
) -> Result<(), String> {
    let mut vault = read_vault_file(&app, &passphrase)?.unwrap_or(AppVault {
        version: VAULT_VERSION,
        hosts: Vec::new(),
        credentials: Vec::new(),
    });
    let host_id = host.id.clone();
    let host_auth_method = host.auth_method.clone();

    if let Some(existing_host) = vault.hosts.iter_mut().find(|item| item.id == host_id) {
        *existing_host = host;
    } else {
        vault.hosts.insert(0, host);
    }

    if credential_action == "preserve" {
        return write_vault_file(&app, &passphrase, &vault);
    }

    if credential_action == "clear" || host_auth_method == "none" {
        vault
            .credentials
            .retain(|credential| credential.host_id != host_id);

        return write_vault_file(&app, &passphrase, &vault);
    }

    let has_private_key = private_key_path.as_ref().is_some_and(|value| !value.is_empty())
        || private_key_content.as_ref().is_some_and(|value| !value.is_empty());
    let auth_type = if has_private_key {
        Some(String::from("privateKey"))
    } else if password.as_ref().is_some_and(|value| !value.is_empty()) {
        Some(String::from("password"))
    } else {
        None
    };

    if let Some(auth_type) = auth_type {
        if let Some(existing_credential) = vault
            .credentials
            .iter_mut()
            .find(|credential| credential.host_id == host_id)
        {
            existing_credential.auth_type = auth_type;
            existing_credential.password = password.filter(|value| !value.is_empty());
            existing_credential.private_key_path =
                private_key_path.filter(|value| !value.is_empty());
            existing_credential.private_key_content =
                private_key_content.filter(|value| !value.is_empty());
            existing_credential.private_key_storage_mode = private_key_storage_mode;
        } else {
            vault.credentials.push(CredentialRecord {
                host_id,
                auth_type,
                password: password.filter(|value| !value.is_empty()),
                private_key_path: private_key_path.filter(|value| !value.is_empty()),
                private_key_content: private_key_content.filter(|value| !value.is_empty()),
                private_key_storage_mode,
            });
        }
    }

    write_vault_file(&app, &passphrase, &vault)
}

#[tauri::command]
pub fn delete_vault_host(
    app: AppHandle,
    passphrase: String,
    host_id: String,
) -> Result<(), String> {
    let mut vault = read_vault_file(&app, &passphrase)?.unwrap_or(AppVault {
        version: VAULT_VERSION,
        hosts: Vec::new(),
        credentials: Vec::new(),
    });

    vault.hosts.retain(|host| host.id != host_id);
    vault
        .credentials
        .retain(|credential| credential.host_id != host_id);

    write_vault_file(&app, &passphrase, &vault)
}
