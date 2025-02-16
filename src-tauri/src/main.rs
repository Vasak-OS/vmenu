// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu_manager;
#[path = "./commands/icons.rs"]
mod icons;
#[path = "./commands/runner.rs"]
mod runner;

use icons::*;
use runner::*;
use tauri::Manager;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            menu_manager::get_menu_items,
            get_icon_base64,
            get_symbol_base64,
            open_app
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            
            // Manejar la pérdida de foco y la tecla Escape
            window.on_window_event(|event| {
                match event {
                    tauri::WindowEvent::Focused(is_focused) => {
                        if !is_focused {
                            std::process::exit(0);
                        }
                    },
                    tauri::WindowEvent::CloseRequested { .. } => {
                        std::process::exit(0);
                    },
                    _ => {}
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
