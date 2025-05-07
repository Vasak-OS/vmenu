// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu_manager;
#[path = "./commands/runner.rs"]
mod runner;
#[path = "./commands/session.rs"]
mod session;


use tauri::Manager;
use tauri_plugin_vicons;
use tauri_plugin_user_data;
use runner::*;  
use session::*;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main")
                .expect("ventana principal no encontrada");
            
            window.on_window_event(move |event| {
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
        .plugin(tauri_plugin_vicons::init())
        .plugin(tauri_plugin_user_data::init())
        .invoke_handler(tauri::generate_handler![
            menu_manager::get_menu_items,
            open_app,
            logout,
            reboot,
            shutdown,
            suspend
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
