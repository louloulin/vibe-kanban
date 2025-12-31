pub mod deployment;
pub mod projects;
pub mod tasks;
pub mod executors;
pub mod filesystem;
pub mod config;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
}

#[tauri::command]
pub async fn health_check() -> Result<HealthCheckResponse, crate::error::DesktopError> {
    Ok(HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
