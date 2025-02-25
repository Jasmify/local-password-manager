// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(_e) = local_password_manager_lib::run() {
        // 致命的なエラーの場合は、アプリケーションを終了
        std::process::exit(1);
    }
}
