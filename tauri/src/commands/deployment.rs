use crate::error::{DesktopError, Result};
use deployment::Deployment;
use serde::Serialize;
use std::sync::Mutex;
use tauri::{AppHandle, State};

#[derive(Debug, Serialize)]
pub struct DeploymentInfo {
    pub assets_dir: String,
    pub is_initialized: bool,
}

// Global state for deployment
pub struct DeploymentState(pub Mutex<Option<Deployment>>);

impl DeploymentState {
    pub fn new() -> Self {
        Self(Mutex::new(None))
    }
}

#[tauri::command]
pub async fn get_deployment_info(
    state: State<'_, DeploymentState>,
) -> Result<DeploymentInfo, DesktopError> {
    let deployment = state.0.lock().unwrap();
    let is_initialized = deployment.is_some();

    Ok(DeploymentInfo {
        assets_dir: utils::assets::asset_dir()
            .to_str()
            .unwrap_or("")
            .to_string(),
        is_initialized,
    })
}

#[tauri::command]
pub async fn initialize_deployment(
    app_handle: AppHandle,
    state: State<'_, DeploymentState>,
) -> Result<String, DesktopError> {
    // Initialize deployment (similar to server main.rs)
    let deployment_impl = deployment::DeploymentImpl::new()
        .await
        .map_err(|e| DesktopError::Deployment(e.to_string()))?;

    // Store deployment in state
    {
        let mut deployment_guard = state.0.lock().unwrap();
        *deployment_guard = Some(deployment_impl.clone());
    }

    // Cleanup tasks
    deployment_impl
        .container()
        .cleanup_orphan_executions()
        .await
        .map_err(|e| DesktopError::Deployment(e.to_string()))?;

    deployment_impl
        .container()
        .backfill_before_head_commits()
        .await
        .map_err(|e| DesktopError::Deployment(e.to_string()))?;

    deployment_impl
        .container()
        .backfill_repo_names()
        .await
        .map_err(|e| DesktopError::Deployment(e.to_string()))?;

    deployment_impl.spawn_pr_monitor_service().await;
    deployment_impl
        .track_if_analytics_allowed("session_start", serde_json::json!({}))
        .await;

    Ok("Deployment initialized successfully".to_string())
}
