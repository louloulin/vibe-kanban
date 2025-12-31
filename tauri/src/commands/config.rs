use crate::error::{DesktopError, Result};
use serde_json::Value;
use std::sync::Mutex;
use tauri::State;

pub struct ConfigState(pub Mutex<Option<Value>>);

impl ConfigState {
    pub fn new() -> Self {
        Self(Mutex::new(None))
    }
}

#[tauri::command]
pub async fn get_config(
    state: State<'_, ConfigState>,
) -> Result<Value, DesktopError> {
    let config = state.0.lock().unwrap();
    Ok(config.clone().unwrap_or_else(|| serde_json::json!({})))
}

#[tauri::command]
pub async fn update_config(
    config: Value,
    state: State<'_, ConfigState>,
) -> Result<(), DesktopError> {
    let mut config_guard = state.0.lock().unwrap();
    *config_guard = Some(config);
    Ok(())
}
