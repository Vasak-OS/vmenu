use std::process::Command;

#[tauri::command]
pub async fn logout() -> Result<(), String> {
    Command::new("loginctl")
        .args(["terminate-session", "$XDG_SESSION_ID"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn reboot() -> Result<(), String> {
    Command::new("systemctl")
        .args(["reboot"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn shutdown() -> Result<(), String> {
    Command::new("systemctl")
        .args(["poweroff"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn suspend() -> Result<(), String> {
    Command::new("systemctl")
        .args(["suspend"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
} 