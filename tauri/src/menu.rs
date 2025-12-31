use tauri::{menu::Menu, AppHandle};

pub fn create_menu(app_handle: &AppHandle) -> tauri::Result<Menu> {
    #[cfg(target_os = "macos")]
    {
        use tauri::menu::{MenuItem, PredefinedMenuItem, Submenu};

        let app_menu = Submenu::new(
            app_handle,
            "Vibe Kanban",
            true,
        )?;

        let file_menu = Submenu::new(
            app_handle,
            "File",
            true,
        )?;

        let edit_menu = Submenu::new(
            app_handle,
            "Edit",
            true,
        )?;

        let view_menu = Submenu::new(
            app_handle,
            "View",
            true,
        )?;

        let window_menu = Submenu::new(
            app_handle,
            "Window",
            true,
        )?;

        let help_menu = Submenu::new(
            app_handle,
            "Help",
            true,
        )?;

        let menu = Menu::with_items(app_handle, &[
            &app_menu,
            &file_menu,
            &edit_menu,
            &view_menu,
            &window_menu,
            &help_menu,
        ])?;

        Ok(menu)
    }

    #[cfg(not(target_os = "macos"))]
    {
        let menu = Menu::new(app_handle)?;
        Ok(menu)
    }
}
