// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tql_internal::Release;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn fetch_releases() -> Vec<Release> {
    tql_internal::fetch_releases().await.unwrap()
}

#[tauri::command]
async fn create_instance(name: &str, version: Release) -> Result<(), ()> {
    tql_internal::create_instance(name, version)
        .await
        .map_err(|_| ())
}

#[tauri::command]
async fn instance_names() -> Vec<String> {
    tql_internal::instance_names()
}

#[tauri::command]
async fn play_instance(name: &str) -> Result<(), &'static str> {
    tql_internal::play_instance(name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_releases,
            create_instance,
            instance_names,
            play_instance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}