use muda::{Menu, MenuItem, PredefinedMenuItem, Submenu, MenuEvent};
use crossbeam_channel;
use crate::{App, Screen, Nav};

/// Create native menu bar for the application
pub fn create_native_menu() -> (Menu, crossbeam_channel::Receiver<MenuEvent>) {
    let menu = Menu::new();
    
    // Application menu (shows app name on macOS)
    let app_submenu = Submenu::new("Brutal Internal", true);
    let about_app_item = MenuItem::with_id("about_app", "About Brutal Internal", true, None);
    let preferences_item = MenuItem::with_id("preferences", "Preferences", true, None);
    
    app_submenu.append_items(&[
        &about_app_item,
        &PredefinedMenuItem::separator(),
        &preferences_item,
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::services(Some("Services")),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::hide(Some("Hide Brutal Internal")),
        &PredefinedMenuItem::hide_others(Some("Hide Others")),
        &PredefinedMenuItem::show_all(Some("Show All")),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::quit(Some("Quit Brutal Internal")),
    ]).unwrap();
    
    // File menu
    let file_submenu = Submenu::new("File", true);
    let new_item = MenuItem::with_id("new", "New Project", true, None);
    let open_item = MenuItem::with_id("open", "Open Project", true, None);
    let save_item = MenuItem::with_id("save", "Save Project", true, None);
    let export_item = MenuItem::with_id("export", "Export Data", true, None);
    let import_item = MenuItem::with_id("import", "Import Data", true, None);
    
    file_submenu.append_items(&[
        &new_item,
        &open_item,
        &PredefinedMenuItem::separator(),
        &save_item,
        &PredefinedMenuItem::separator(),
        &export_item,
        &import_item,
    ]).unwrap();
    
    // Edit menu
    let edit_submenu = Submenu::new("Edit", true);
    
    edit_submenu.append_items(&[
        &PredefinedMenuItem::undo(Some("Undo")),
        &PredefinedMenuItem::redo(Some("Redo")),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::cut(Some("Cut")),
        &PredefinedMenuItem::copy(Some("Copy")),
        &PredefinedMenuItem::paste(Some("Paste")),
    ]).unwrap();
    
    // View menu
    let view_submenu = Submenu::new("View", true);
    let dashboard_item = MenuItem::with_id("view_dashboard", "Dashboard", true, None);
    let reports_item = MenuItem::with_id("view_reports", "Reports", true, None);
    let data_item = MenuItem::with_id("view_data", "Data", true, None);
    let tasks_item = MenuItem::with_id("view_tasks", "Tasks", true, None);
    let settings_item = MenuItem::with_id("view_settings", "Settings", true, None);
    
    view_submenu.append_items(&[
        &dashboard_item,
        &reports_item,
        &data_item,
        &tasks_item,
        &settings_item,
    ]).unwrap();
    
    // Help menu
    let help_submenu = Submenu::new("Help", true);
    let about_item = MenuItem::with_id("about", "About Brutal Internal", true, None);
    let docs_item = MenuItem::with_id("docs", "Documentation", true, None);
    
    help_submenu.append_items(&[
        &about_item,
        &docs_item,
    ]).unwrap();
    
    // Add submenus to main menu
    menu.append_items(&[
        &app_submenu,
        &file_submenu,
        &edit_submenu,
        &view_submenu,
        &help_submenu,
    ]).unwrap();
    
    // Note: macOS menu initialization will happen later in update() method
    #[cfg(target_os = "macos")]
    {
        println!("Menu created, will initialize for macOS after app starts...");
    }
    
    #[cfg(target_os = "windows")]
    {
        // Note: Windows implementation would require window handle
        println!("Windows menu - would need window handle");
    }
    
    #[cfg(target_os = "linux")]
    {
        // Note: Linux implementation would require window handle
        println!("Linux menu - would need window handle");
    }
    
    // Create menu event receiver
    let menu_channel = MenuEvent::receiver().clone();
    
    (menu, menu_channel)
}

/// Handle menu events for the application
pub fn handle_menu_events(app: &mut App, ctx: &eframe::egui::Context) {
    if let Some(receiver) = &app.menu_receiver {
        let mut events_processed = false;
        while let Ok(event) = receiver.try_recv() {
            events_processed = true;
            let menu_id = event.id().0.as_str();
            
            match menu_id {
                "new" => {
                    println!("New Project menu selected");
                    // Reset to startup screen for demo
                    app.screen = Screen::Startup;
                },
                "open" => {
                    println!("Open Project menu selected");
                    // Could open file dialog here
                },
                "save" => {
                    println!("Save Project menu selected");
                    // Could save current state here
                },
                "export" => {
                    println!("Export Data menu selected");
                    // Could export data here
                },
                "import" => {
                    println!("Import Data menu selected");
                    // Could import data here
                },
                "preferences" => {
                    println!("Preferences menu selected");
                    // Switch to settings
                    app.nav = Nav::Settings;
                    app.screen = Screen::Dashboard;
                },
                "about" => {
                    println!("About menu selected");
                    // Could show about dialog
                },
                "about_app" => {
                    println!("About Brutal Internal menu selected");
                    // Could show about dialog
                },
                "view_dashboard" => {
                    app.nav = Nav::Dashboard;
                    app.screen = Screen::Dashboard;
                },
                "view_reports" => {
                    app.nav = Nav::Reports;
                    app.screen = Screen::Dashboard;
                },
                "view_data" => {
                    app.nav = Nav::Data;
                    app.screen = Screen::Dashboard;
                },
                "view_tasks" => {
                    app.nav = Nav::Tasks;
                    app.screen = Screen::Dashboard;
                },
                "view_settings" => {
                    app.nav = Nav::Settings;
                    app.screen = Screen::Dashboard;
                },
                _ => {
                    println!("Unhandled menu event: {:?}", menu_id);
                },
            }
        }
        
        // Request repaint if any events were processed for immediate UI update
        if events_processed {
            ctx.request_repaint();
        }
    }
}