use crate::error::{DesktopError, Result};
use crate::commands::deployment::DeploymentState;
use db::models::project::Project;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        ProjectResponse {
            id: project.id.to_string(),
            name: project.name,
            description: project.description,
            created_at: project.created_at.to_rfc3339(),
            updated_at: project.updated_at.to_rfc3339(),
        }
    }
}

#[tauri::command]
pub async fn get_projects(
    state: State<'_, DeploymentState>,
) -> Result<Vec<ProjectResponse>, DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let projects = deployment
        .db()
        .list_projects()
        .await
        .map_err(|e| DesktopError::Database(e.to_string()))?;

    Ok(projects.into_iter().map(ProjectResponse::from).collect())
}

#[tauri::command]
pub async fn get_project(
    id: String,
    state: State<'_, DeploymentState>,
) -> Result<ProjectResponse, DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let project_uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| DesktopError::NotFound(format!("Invalid project ID: {}", id)))?;

    let project = deployment
        .db()
        .get_project(project_uuid)
        .await
        .map_err(|e| DesktopError::Database(e.to_string()))?
        .ok_or_else(|| DesktopError::NotFound(format!("Project not found: {}", id)))?;

    Ok(ProjectResponse::from(project))
}

#[tauri::command]
pub async fn create_project(
    name: String,
    description: Option<String>,
    state: State<'_, DeploymentState>,
) -> Result<ProjectResponse, DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let project = deployment
        .create_project(name, description)
        .await
        .map_err(|e| DesktopError::Deployment(e.to_string()))?;

    Ok(ProjectResponse::from(project))
}

#[tauri::command]
pub async fn update_project(
    id: String,
    name: Option<String>,
    description: Option<String>,
    state: State<'_, DeploymentState>,
) -> Result<ProjectResponse, DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let project_uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| DesktopError::NotFound(format!("Invalid project ID: {}", id)))?;

    // Update logic here
    let project = deployment
        .db()
        .get_project(project_uuid)
        .await
        .map_err(|e| DesktopError::Database(e.to_string()))?
        .ok_or_else(|| DesktopError::NotFound(format!("Project not found: {}", id)))?;

    Ok(ProjectResponse::from(project))
}

#[tauri::command]
pub async fn delete_project(
    id: String,
    state: State<'_, DeploymentState>,
) -> Result<(), DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let project_uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| DesktopError::NotFound(format!("Invalid project ID: {}", id)))?;

    deployment
        .db()
        .delete_project(project_uuid)
        .await
        .map_err(|e| DesktopError::Database(e.to_string()))?;

    Ok(())
}
