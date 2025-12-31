use crate::error::{DesktopError, Result};
use std::fs;
use std::path::Path;

#[tauri::command]
pub async fn read_file(path: String) -> Result<String, DesktopError> {
    let content = fs::read_to_string(&path)
        .map_err(|e| DesktopError::Io(e))?;
    Ok(content)
}

#[tauri::command]
pub async fn write_file(path: String, content: String) -> Result<(), DesktopError> {
    fs::write(&path, content)
        .map_err(|e| DesktopError::Io(e))?;
    Ok(())
}

#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<String>, DesktopError> {
    let entries = fs::read_dir(&path)
        .map_err(|e| DesktopError::Io(e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect();

    Ok(entries)
}
