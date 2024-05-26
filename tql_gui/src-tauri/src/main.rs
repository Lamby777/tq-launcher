// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tql_internal::Release;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn fetch_versions() -> Vec<Release> {
    tql_internal::fetch_versions().await.unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_versions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
