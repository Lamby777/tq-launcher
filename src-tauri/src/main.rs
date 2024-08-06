// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use tql_internal::{InstanceInfo, Release};

#[tauri::command]
async fn fetch_releases() -> Result<Vec<Release>, ()> {
    tql_internal::fetch_releases().await.map_err(|_| ())
}

#[tauri::command]
async fn create_instance(name: &str, version: Release) -> Result<(), String> {
    tql_internal::create_instance(name, version)
        .await
        .map_err(|v| v.to_string())
}

#[tauri::command]
fn instance_map() -> HashMap<String, InstanceInfo> {
    tql_internal::instance_map()
}

#[tauri::command]
async fn play_instance(name: &str) -> Result<(), &'static str> {
    tql_internal::play_instance(name)
}

#[tauri::command]
async fn alter_instance(name: &str, flags: &str) -> Result<(), &'static str> {
    // TODO use a struct for options
    tql_internal::alter_instance(name, flags)
}

#[tauri::command]
async fn delete_instance(name: &str) -> Result<(), &'static str> {
    tql_internal::delete_instance(name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_releases,
            create_instance,
            delete_instance,
            instance_map,
            play_instance,
            alter_instance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
