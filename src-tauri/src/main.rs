// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu_manager;
#[path = "./commands/icons.rs"]
mod icons;
#[path = "./commands/runner.rs"]
mod runner;
#[path = "./commands/session.rs"]
mod session;
#[path = "./commands/user.rs"]
mod user;


use tauri::Manager;
use icons::*;
use runner::*;
use session::*;
use user::*;

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
        .invoke_handler(tauri::generate_handler![
            menu_manager::get_menu_items,
            get_icon_base64,
            get_symbol_base64,
            open_app,
            logout,
            reboot,
            shutdown,
            suspend,
            get_user_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
