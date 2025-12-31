// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod menu;
mod tray;
mod error;

use tauri::Manager;
use tracing_subscriber::{EnvFilter, prelude::*};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let filter_string = format!(
        "warn,vibe_kanban_desktop={level},server={level},services={level},db={level},executors={level},deployment={level},local_deployment={level},utils={level}",
        level = log_level
    );
    let env_filter = EnvFilter::try_new(filter_string)?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(env_filter))
        .init();

    Ok(())
}

fn main() {
    // Run async initialization
    if let Err(e) = tokio::runtime::Runtime::new()
        .expect("Failed to create runtime")
        .block_on(run())
    {
        eprintln!("Initialization error: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_title_bar_style(TitleBarStyle::Overlay);
            }

            // Initialize system tray
            tray::create_tray(app.handle())?;

            // Initialize app state
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // Initialize deployment and other services here
                if let Err(e) = commands::deployment::initialize_deployment(app_handle).await {
                    tracing::error!("Failed to initialize deployment: {}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Health check
            commands::health_check,
            // Deployment commands
            commands::deployment::get_deployment_info,
            commands::deployment::initialize_deployment,
            // Project commands
            commands::projects::get_projects,
            commands::projects::get_project,
            commands::projects::create_project,
            commands::projects::update_project,
            commands::projects::delete_project,
            // Task commands
            commands::tasks::get_tasks,
            commands::tasks::get_task,
            commands::tasks::create_task,
            commands::tasks::update_task,
            commands::tasks::delete_task,
            // Executor commands
            commands::executors::get_executors,
            commands::executors::get_executor_config,
            commands::executors::update_executor_config,
            // File system commands
            commands::filesystem::read_file,
            commands::filesystem::write_file,
            commands::filesystem::list_directory,
            // Config commands
            commands::config::get_config,
            commands::config::update_config,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                // Prevent window from closing immediately
                api.prevent_exit();
            }
        });
}
