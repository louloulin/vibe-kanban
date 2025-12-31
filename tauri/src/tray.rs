use tauri::{AppHandle, Manager, tray::TrayIconBuilder, menu::Menu};

pub fn create_tray(app_handle: &AppHandle) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let show = tauri::menu::MenuItem::with_id(app_handle, "show", "Show", true, None::<&str>)?;
        let hide = tauri::menu::MenuItem::with_id(app_handle, "hide", "Hide", true, None::<&str>)?;
        let quit = tauri::menu::MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;

        let menu = Menu::with_items(app_handle, &[&show, &hide, &quit])?;

        let _tray = TrayIconBuilder::new()
            .menu(&menu)
            .menu_on_left_click(false)
            .title("Vibe Kanban")
            .tooltip("Vibe Kanban")
            .build(app_handle)?;
    }

    #[cfg(target_os = "windows")]
    {
        let show = tauri::menu::MenuItem::with_id(app_handle, "show", "Show", true, None::<&str>)?;
        let quit = tauri::menu::MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;

        let menu = Menu::with_items(app_handle, &[&show, &quit])?;

        let _tray = TrayIconBuilder::new()
            .menu(&menu)
            .menu_on_left_click(false)
            .tooltip("Vibe Kanban")
            .build(app_handle)?;
    }

    #[cfg(target_os = "linux")]
    {
        let show = tauri::menu::MenuItem::with_id(app_handle, "show", "Show", true, None::<&str>)?;
        let quit = tauri::menu::MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;

        let menu = Menu::with_items(app_handle, &[&show, &quit])?;

        let _tray = TrayIconBuilder::new()
            .menu(&menu)
            .menu_on_left_click(false)
            .tooltip("Vibe Kanban")
            .build(app_handle)?;
    }

    Ok(())
}
