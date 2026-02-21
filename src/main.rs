#![windows_subsystem = "windows"] 
// the line above tells Rust to build a Windows GUI app without a console window.
// It is important for Windows builds, but does not affect other platforms
//  It tells the linker to use /SUBSYSTEM:WINDOWS instead of /SUBSYSTEM:CONSOLE, which suppresses the default cmd.exe window that pops up when running a GUI app.
// Reference: https://doc.rust-lang.org/reference/runtime.html#the-windows_subsystem-attribute

use eframe::egui;
use egui::IconData;
use muda::{Menu, MenuEvent};
use crossbeam_channel;

mod menu;
mod startup;
mod dashboard;
mod theme;
mod widgets;

/// Load the app icon from embedded image data
fn load_app_icon() -> IconData {
    // Load icon from assets - you can use PNG, ICO, or other formats
    // For best results, use a square image (e.g., 32x32, 64x64, 128x128, 256x256)
    let icon_bytes = include_bytes!("../assets/images/app_icon.png"); // Adjust path as needed
    
    // Decode the image
    match image::load_from_memory(icon_bytes) {
        Ok(image) => {
            let rgba = image.to_rgba8();
            let (width, height) = rgba.dimensions();
            
            IconData {
                rgba: rgba.into_raw(),
                width,
                height,
            }
        }
        Err(_) => {
            // Fallback: create a simple colored square icon if image loading fails
            let size = 32;
            let pixel_count = (size * size) as usize;
            let mut rgba = Vec::with_capacity(pixel_count * 4);
            for _ in 0..pixel_count {
                rgba.extend_from_slice(&[0xF9, 0x8C, 0x1F, 255]); // Orange color (RGBA)
            }
            
            IconData {
                rgba,
                width: size,
                height: size,
            }
        }
    }
}



fn main() -> eframe::Result<()> {
    // Load app icon from embedded bytes
    let icon_data = load_app_icon();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Brutal Internal")
            .with_inner_size([1180.0, 760.0])
            .with_icon(icon_data),
        ..Default::default()
    };

    // Create native menu (without initializing it yet for macOS)
    let (menu, menu_receiver) = menu::create_native_menu();
    

    eframe::run_native(
        "Brutal Internal",
        options,
        Box::new(move |cc| {
            // 1) Fonts must be installed before any drawing
            theme::install_fonts(&cc.egui_ctx);

            // 2) Global visual theme
            theme::install_theme(&cc.egui_ctx);

            // 3) Create app with menu receiver
            let mut app = App::default();
            app.menu_receiver = Some(menu_receiver);
            app.menu = Some(menu); // Keep menu alive
            
            Ok(Box::new(app))
        }),
    )
}

pub struct App {
    pub screen: Screen,
    pub nav: Nav,
    // pretend state (you'll replace these with real data)
    pub active_users: u32,
    pub pending_tasks: u32,
    pub revenue: i64,
    pub new_orders: u32,
    pub status_online: bool,
    // Menu event receiver
    pub menu_receiver: Option<crossbeam_channel::Receiver<MenuEvent>>,
    // Keep menu alive
    pub menu: Option<Menu>,
    // Track if menu has been initialized for macOS
    pub menu_initialized: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::default(),
            nav: Nav::default(),
            active_users: 0,
            pending_tasks: 0,
            revenue: 0,
            new_orders: 0,
            status_online: false,
            menu_receiver: None,
            menu: None,
            menu_initialized: false,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    #[default]
    AlternativeSplash,  // Changed: Image splash is now the default
    Startup,            // Original text-based splash
    Dashboard,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Nav {
    #[default]
    Dashboard,
    Reports,
    Data,
    Tasks,
    Settings,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Initialize menu for macOS on first update call
        #[cfg(target_os = "macos")]
        if !self.menu_initialized {
            if let Some(ref menu) = self.menu {
                println!("Initializing menu for macOS NSApp in update method...");
                menu.init_for_nsapp();
                println!("Menu initialized successfully!");
                self.menu_initialized = true;
            }
        }
        
        // Handle menu events (this will request repaint if events are processed)
        menu::handle_menu_events(self, ctx);
        
        // Ensure continuous updates for responsive UI
        ctx.request_repaint_after(std::time::Duration::from_millis(16)); // ~60fps
        
        // initialize some default demo values (only once-ish)
        if self.active_users == 0 && self.pending_tasks == 0 && self.revenue == 0 {
            self.active_users = 128;
            self.pending_tasks = 7;
            self.revenue = 42300;
            self.new_orders = 56;
            self.status_online = true;
        }

        match self.screen {
            Screen::Startup => startup::render_startup(self, ctx),
            Screen::AlternativeSplash => startup::render_alternative_splash(self, ctx),
            Screen::Dashboard => dashboard::render_dashboard(self, ctx),
        }
    }
}

// Re-export commonly used items from widgets module
pub use widgets::{Icon, icon_glyph};