#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod tests;
mod measurement;
mod commands;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::convert
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
