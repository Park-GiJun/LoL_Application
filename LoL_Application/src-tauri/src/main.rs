// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod save_match;

use save_match::read_log_file::read_log_file;

#[tauri::command]
fn get_log_file() -> Result<(Vec<(String, String)>, Vec<(String, String)>), String> {
    let path = "C:\\Riot Games\\League of Legends\\Logs\\GameLogs\\2024-07-06T22-25-19\\2024-07-06T22-25-19_r3dlog.txt";
    read_log_file(path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_log_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
