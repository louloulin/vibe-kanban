use crate::error::{DesktopError, Result};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ExecutorInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub configured: bool,
}

#[tauri::command]
pub async fn get_executors() -> Result<Vec<ExecutorInfo>, DesktopError> {
    // Return list of available executors
    Ok(vec![
        ExecutorInfo {
            name: "claude-code".to_string(),
            display_name: "Claude Code".to_string(),
            description: "Anthropic's Claude Code AI assistant".to_string(),
            configured: true,
        },
        ExecutorInfo {
            name: "codex".to_string(),
            display_name: "OpenAI Codex".to_string(),
            description: "OpenAI's Codex AI assistant".to_string(),
            configured: false,
        },
        ExecutorInfo {
            name: "gemini-cli".to_string(),
            display_name: "Gemini CLI".to_string(),
            description: "Google's Gemini AI assistant".to_string(),
            configured: false,
        },
    ])
}

#[tauri::command]
pub async fn get_executor_config(
    name: String,
) -> Result<HashMap<String, serde_json::Value>, DesktopError> {
    // Return executor configuration
    Ok(HashMap::new())
}

#[tauri::command]
pub async fn update_executor_config(
    name: String,
    config: HashMap<String, serde_json::Value>,
) -> Result<(), DesktopError> {
    // Update executor configuration
    Ok(())
}
