// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn fetch_versions() -> String {
    let versions = tql_internal::fetch_versions().await.unwrap();
    let versions = versions
        .into_iter()
        .map(|r| r.name.unwrap_or("Unnamed".to_string()))
        .collect::<Vec<_>>();
    format!("{:?}", versions.join(", "))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_versions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
