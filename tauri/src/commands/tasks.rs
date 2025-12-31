use crate::error::{DesktopError, Result};
use crate::commands::deployment::DeploymentState;
use db::models::task::Task;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        TaskResponse {
            id: task.id.to_string(),
            project_id: task.project_id.to_string(),
            title: task.title,
            description: task.description,
            status: task.status.to_string(),
            created_at: task.created_at.to_rfc3339(),
            updated_at: task.updated_at.to_rfc3339(),
        }
    }
}

#[tauri::command]
pub async fn get_tasks(
    project_id: String,
    state: State<'_, DeploymentState>,
) -> Result<Vec<TaskResponse>, DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let project_uuid = uuid::Uuid::parse_str(&project_id)
        .map_err(|_| DesktopError::NotFound(format!("Invalid project ID: {}", project_id)))?;

    let tasks = deployment
        .db()
        .list_tasks(project_uuid)
        .await
        .map_err(|e| DesktopError::Database(e.to_string()))?;

    Ok(tasks.into_iter().map(TaskResponse::from).collect())
}

#[tauri::command]
pub async fn get_task(
    id: String,
    state: State<'_, DeploymentState>,
) -> Result<TaskResponse, DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let task_uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| DesktopError::NotFound(format!("Invalid task ID: {}", id)))?;

    let task = deployment
        .db()
        .get_task(task_uuid)
        .await
        .map_err(|e| DesktopError::Database(e.to_string()))?
        .ok_or_else(|| DesktopError::NotFound(format!("Task not found: {}", id)))?;

    Ok(TaskResponse::from(task))
}

#[tauri::command]
pub async fn create_task(
    project_id: String,
    title: String,
    description: Option<String>,
    state: State<'_, DeploymentState>,
) -> Result<TaskResponse, DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let project_uuid = uuid::Uuid::parse_str(&project_id)
        .map_err(|_| DesktopError::NotFound(format!("Invalid project ID: {}", project_id)))?;

    let task = deployment
        .create_task(project_uuid, title, description)
        .await
        .map_err(|e| DesktopError::Deployment(e.to_string()))?;

    Ok(TaskResponse::from(task))
}

#[tauri::command]
pub async fn update_task(
    id: String,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    state: State<'_, DeploymentState>,
) -> Result<TaskResponse, DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let task_uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| DesktopError::NotFound(format!("Invalid task ID: {}", id)))?;

    let task = deployment
        .db()
        .get_task(task_uuid)
        .await
        .map_err(|e| DesktopError::Database(e.to_string()))?
        .ok_or_else(|| DesktopError::NotFound(format!("Task not found: {}", id)))?;

    Ok(TaskResponse::from(task))
}

#[tauri::command]
pub async fn delete_task(
    id: String,
    state: State<'_, DeploymentState>,
) -> Result<(), DesktopError> {
    let deployment = state.0.lock().unwrap();
    let deployment = deployment
        .as_ref()
        .ok_or_else(|| DesktopError::Deployment("Not initialized".to_string()))?;

    let task_uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| DesktopError::NotFound(format!("Invalid task ID: {}", id)))?;

    deployment
        .db()
        .delete_task(task_uuid)
        .await
        .map_err(|e| DesktopError::Database(e.to_string()))?;

    Ok(())
}
