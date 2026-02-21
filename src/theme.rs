use eframe::egui;

/// Color constants for the brutalist theme
pub struct BrutalColors;

impl BrutalColors {
    // Core background colors
    pub const WINDOW: egui::Color32 = egui::Color32::from_gray(8);
    pub const PANEL: egui::Color32 = egui::Color32::from_gray(8);
    pub const FAINT_BG: egui::Color32 = egui::Color32::from_gray(12);
    pub const BLACK: egui::Color32 = egui::Color32::BLACK;
    pub const CARD_BG: egui::Color32 = egui::Color32::from_gray(18);
    pub const PANEL_BG: egui::Color32 = egui::Color32::from_gray(14);
    
    // Text colors
    pub const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_gray(240);
    pub const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_gray(200);
    pub const TEXT_WEAK: egui::Color32 = egui::Color32::from_gray(160);
    pub const TEXT_WHITE: egui::Color32 = egui::Color32::WHITE;
    
    // Accent colors
    pub const WHITE: egui::Color32 = egui::Color32::WHITE;
    
    // pub const ORANGE: egui::Color32 = egui::Color32::from_rgb(0xF9, 0x8C, 0x1F);
    // pub const ORANGE: egui::Color32 = egui::Color32::from_rgb(0xFC, 0x7F, 0x1C); // #fc7f1c
    pub const ORANGE: egui::Color32 = egui::Color32::from_rgb(0xFC, 0x77, 0x19); // #fc7719
    pub const GREEN: egui::Color32 = egui::Color32::from_rgb(0, 255, 0);
    pub const RED: egui::Color32 = egui::Color32::from_rgb(255, 0, 0);
    pub const RED_DARK: egui::Color32 = egui::Color32::from_rgb(220, 20, 20);
    
    // Widget specific colors
    pub const WIDGET_NONINTERACTIVE: egui::Color32 = egui::Color32::from_gray(20);
    pub const WIDGET_INACTIVE: egui::Color32 = egui::Color32::from_gray(25);
    pub const WIDGET_HOVERED: egui::Color32 = egui::Color32::from_gray(35);
    pub const WIDGET_ACTIVE: egui::Color32 = egui::Color32::from_gray(45);
    pub const WIDGET_SELECTED: egui::Color32 = egui::Color32::from_gray(40);
    pub const WIDGET_UNSELECTED: egui::Color32 = egui::Color32::from_gray(22);
    
    // Status colors
    pub const STATUS_ONLINE_BG: egui::Color32 = egui::Color32::from_rgb(70, 140, 70);
    pub const STATUS_OFFLINE_BG: egui::Color32 = egui::Color32::from_rgb(160, 70, 70);
    
    // Border colors
    pub const BORDER_MAIN: egui::Color32 = egui::Color32::from_gray(190);
    pub const BORDER_INACTIVE: egui::Color32 = egui::Color32::from_gray(100);
    pub const BORDER_ACTIVE: egui::Color32 = egui::Color32::WHITE;
}

/// Stroke constants for brutalist design
pub struct BrutalStrokes;

impl BrutalStrokes {
    pub fn main() -> egui::Stroke {
        egui::Stroke::new(2.0, BrutalColors::BORDER_MAIN)
    }
    
    pub fn inactive() -> egui::Stroke {
        egui::Stroke::new(2.0, BrutalColors::BORDER_INACTIVE)
    }
    
    pub fn active() -> egui::Stroke {
        egui::Stroke::new(2.0, BrutalColors::BORDER_ACTIVE)
    }
    
    pub fn red() -> egui::Stroke {
        egui::Stroke::new(2.0, BrutalColors::RED_DARK)
    }
    
    pub const NONE: egui::Stroke = egui::Stroke::NONE;
}

/// Font sizes and spacing constants
pub struct BrutalSizes;

impl BrutalSizes {
    // Font sizes
    pub const TITLE_LARGE: f32 = 24.0;
    pub const TITLE: f32 = 16.0;
    pub const BODY: f32 = 14.0;
    pub const SMALL: f32 = 12.0;
    pub const TINY: f32 = 10.0;
    pub const STAT_VALUE: f32 = 22.0;
    
    // Spacing
    pub const MARGIN_SMALL: f32 = 8.0;
    pub const MARGIN_CONTENT: f32 = 10.0;
    pub const MARGIN_MEDIUM: f32 = 12.0;
    pub const MARGIN_SECTION: f32 = 14.0;
    pub const MARGIN_LARGE: f32 = 16.0;
    pub const MARGIN_HUGE: f32 = 20.0;
    
    // Widget sizes
    pub const BADGE_WIDTH: f32 = 220.0;
    pub const BADGE_STAT_WIDTH: f32 = 260.0;
    pub const BADGE_HEIGHT: f32 = 36.0;
    pub const PROGRESS_HEIGHT: f32 = 18.0;
    pub const BUTTON_MIN_WIDTH: f32 = 80.0;
    pub const BUTTON_MIN_HEIGHT: f32 = 20.0;
    pub const NAV_BUTTON_WIDTH: f32 = 180.0;
    pub const NAV_BUTTON_HEIGHT: f32 = 28.0;
}

/// Install fonts from assets
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Load font files
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Anton-Regular.ttf")));
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/MadimiOne-Regular.ttf")));
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Audiowide-Regular.ttf")));
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Orbitron-VariableFont_wght.ttf")));
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/PressStart2P-Regular.ttf")));
    // let brutal  = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Handjet-VariableFont_ELGR,ELSH,wght.ttf")));
    // let brutal =   std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/NovaMono-Regular.ttf")));
    
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Offside-Regular.ttf")));

    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Iceland-Regular.ttf")));

    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Electrolize-Regular.ttf")));

    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Quantico-Regular.ttf")));
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/TiltNeon-Regular-VariableFont_XROT,YROT.ttf")));
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Baumans-Regular.ttf")));
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/EduNSWACTCursive-VariableFont_wght.ttf")));
    // let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/GoogleSansCode-VariableFont_wght.ttf")));

    let brutal = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Bungee-Regular.ttf")));
    

    


    

    



    
    
    
    
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/CaskaydiaCoveNerdFont-ExtraLightItalic.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/DotGothic16-Regular.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/iMWritingMonoNerdFontMono-Regular.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/IosevkaTermNerdFont-Heavy.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/CascadiaMono.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/SpaceMono-Regular.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/terminal.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/WorkSans-Regular.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Monaco.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Monaco.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Source Code Pro iCursive S12 Regular.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/CascadiaCode-SemiBold.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/Manrope-VariableFont_wght.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/SF-Pro.ttf")));


    
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf")));
    // let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/anonymous_pro_bold.ttf")));

    let mono = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/MonofurNerdFont-Regular.ttf")));





    let icons = std::sync::Arc::new(egui::FontData::from_static(include_bytes!("../assets/fonts/MaterialSymbolsRounded.ttf")));

    

    fonts.font_data.insert("Brutal".to_string(), brutal);
    fonts.font_data.insert("Mono".to_string(), mono);
    fonts.font_data.insert("IconFont".to_string(), icons);

    // Add to families
    fonts
        .families
        .entry(egui::FontFamily::Name("Brutal".into()))
        .or_default()
        .push("Brutal".to_string());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "Mono".to_string());

    fonts
        .families
        .entry(egui::FontFamily::Name("IconFont".into()))
        .or_default()
        .insert(0, "IconFont".to_string());

    ctx.set_fonts(fonts);
}

/// Install global brutalist theme
pub fn install_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals.dark_mode = true;

    // Background colors - darker brutalist theme
    style.visuals.window_fill = BrutalColors::WINDOW;
    style.visuals.panel_fill = BrutalColors::PANEL;
    style.visuals.faint_bg_color = BrutalColors::FAINT_BG;

    // Text colors
    style.visuals.override_text_color = Some(BrutalColors::TEXT_PRIMARY);
    style.visuals.weak_text_color = Some(BrutalColors::TEXT_SECONDARY);

    // Widget colors - better contrast
    style.visuals.widgets.noninteractive.bg_fill = BrutalColors::WIDGET_NONINTERACTIVE;
    style.visuals.widgets.inactive.bg_fill = BrutalColors::WIDGET_INACTIVE;
    style.visuals.widgets.hovered.bg_fill = BrutalColors::WIDGET_HOVERED;
    style.visuals.widgets.active.bg_fill = BrutalColors::WIDGET_ACTIVE;

    // Widget strokes - brutalist borders
    style.visuals.widgets.noninteractive.fg_stroke = BrutalStrokes::inactive();
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(2.0, BrutalColors::BORDER_INACTIVE);
    style.visuals.widgets.hovered.fg_stroke = BrutalStrokes::active();
    style.visuals.widgets.active.fg_stroke = BrutalStrokes::red();

    // Brutalist: zero corner radius everywhere
    style.visuals.widgets.noninteractive.expansion = 0.0;
    style.visuals.widgets.inactive.expansion = 0.0;
    style.visuals.widgets.hovered.expansion = 0.0;
    style.visuals.widgets.active.expansion = 0.0;

    // Remove rounding
    style.visuals.window_corner_radius = egui::CornerRadius::ZERO;
    style.visuals.menu_corner_radius = egui::CornerRadius::ZERO;

    // Spacing & sizing
    style.spacing.button_padding = egui::vec2(10.0, 8.0);
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);

    ctx.set_style(style);
}

/// Create a brutalist frame with zero corner radius
pub fn brutalist_frame(fill: egui::Color32, stroke: egui::Stroke) -> egui::Frame {
    egui::Frame::NONE
        .fill(fill)
        .stroke(stroke)
        .corner_radius(egui::CornerRadius::ZERO)
}