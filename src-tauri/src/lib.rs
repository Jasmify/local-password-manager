mod commands;
mod crypto;
mod database;
mod models;
mod repository;

use anyhow::Result;
use std::env;
use tauri::Manager;

use crypto::{create_key_file, get_key_file_path, AES_KEY_ENV_VAR};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let sqlite_pool = database::setup_database()?;
    let key_file_path = get_key_file_path();
    // 環境変数とキーファイルが存在しない場合、キーを作成
    if env::var(AES_KEY_ENV_VAR).is_err() && !key_file_path.exists() {
        create_key_file(key_file_path);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::insert_form_data,
            commands::get_account_summary,
            commands::get_search_results,
            commands::get_password_info,
            commands::update_account_info,
            commands::delete_account,
        ])
        .setup(|app| {
            app.manage(sqlite_pool);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
