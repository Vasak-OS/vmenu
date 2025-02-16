use std::process::Command;
use serde::{Serialize, Deserialize};
use std::env;
use std::fs;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    username: String,
    full_name: String,
    avatar_base64: String,
    home_dir: String,
}

#[tauri::command]
pub async fn get_user_info() -> Result<UserInfo, String> {
    let username = env::var("USER").unwrap_or_default();
    let home_dir = env::var("HOME").unwrap_or_default();
    
    // Obtener nombre completo
    let full_name = Command::new("getent")
        .args(["passwd", &username])
        .output()
        .map_err(|e| e.to_string())
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .map_err(|e| e.to_string())
                .map(|s| {
                    s.split(':')
                        .nth(4)
                        .unwrap_or(&username)
                        .split(',')
                        .next()
                        .unwrap_or(&username)
                        .to_string()
                })
        })?;

    // Buscar y codificar avatar
    let avatar_paths = [
        format!("{}/.face", home_dir),
        format!("/var/lib/AccountsService/icons/{}", username),
        "/usr/share/pixmaps/faces/default.png".to_string(),
    ];

    let avatar_base64 = avatar_paths
        .iter()
        .find(|path| std::path::Path::new(path).exists())
        .and_then(|path| fs::read(path).ok())
        .map(|bytes| BASE64.encode(bytes))
        .unwrap_or_default();

    Ok(UserInfo {
        username,
        full_name,
        avatar_base64,
        home_dir,
    })
} 