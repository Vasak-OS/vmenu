use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[tauri::command]
pub async fn open_app(path: &str) -> Result<(), String> {
    // Leer el archivo .desktop
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    
    // Buscar la línea Exec=
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("Exec=") {
                let cmd = line.trim_start_matches("Exec=");
                // Eliminar cualquier parámetro después del comando
                let cmd = cmd.split_whitespace().next().unwrap_or(cmd);
                
                Command::new(cmd)
                    .spawn()
                    .map_err(|e| e.to_string())?;
                
                std::process::exit(0);
                return Ok(());
            }
        }
    }
    
    Err("No se encontró el comando ejecutable".to_string())
}