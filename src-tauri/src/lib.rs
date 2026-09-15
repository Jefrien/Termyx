mod ssh;
mod vault;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ssh::SshSessionRegistry::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            vault::delete_vault_host,
            ssh::resize_ssh_session,
            ssh::start_ssh_session,
            ssh::stop_ssh_session,
            ssh::write_ssh_session,
            vault::load_app_vault,
            vault::reset_app_vault,
            vault::save_app_vault,
            vault::upsert_vault_host,
            vault::vault_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
