// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu_manager;
#[path = "./commands/icons.rs"]
mod icons;

use icons::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            menu_manager::get_menu_items,
            get_icon_base64,
            get_symbol_base64
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
