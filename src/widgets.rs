use eframe::egui;
use crate::theme::{BrutalColors, BrutalSizes, BrutalStrokes, brutalist_frame};

/// Icon types for UI glyphs
#[derive(Clone, Copy)]
pub enum Icon {
    Home,
    Chart,
    Database,
    Check,
    Gear,
}

/// Get glyph string for icon using Material Symbols
pub fn icon_glyph(icon: Icon) -> String {
    match icon {
        Icon::Home => "\u{e88a}".to_owned(),       // home Material Symbol
        Icon::Chart => "\u{e26b}".to_owned(),      // bar_chart Material Symbol  
        Icon::Database => "\u{e1db}".to_owned(),   // storage Material Symbol
        Icon::Check => "\u{e5ca}".to_owned(),      // check Material Symbol
        Icon::Gear => "\u{e8b8}".to_owned(),       // settings Material Symbol
    }
}

/// Status badge widget
pub fn badge(ui: &mut egui::Ui, border: egui::Stroke, online: bool) {
    let (text, fill) = if online {
        ("STATUS: ONLINE", BrutalColors::STATUS_ONLINE_BG)
    } else {
        ("STATUS: OFFLINE", BrutalColors::STATUS_OFFLINE_BG)
    };

    let frame = brutalist_frame(fill, border)
        .inner_margin(egui::Margin::same(0));
    
    let desired_size = egui::vec2(BrutalSizes::BADGE_WIDTH, BrutalSizes::BADGE_HEIGHT);
    
    frame.show(ui, |ui| {
        ui.set_min_size(desired_size);
        ui.centered_and_justified(|ui| {
            ui.label(
                egui::RichText::new(text)
                    .color(BrutalColors::BLACK)
                    .strong()
                    .size(BrutalSizes::BODY),
            );
        });
    });

    ui.add_space(BrutalSizes::MARGIN_MEDIUM);
}

/// Stat badge widget
pub fn badge_stat(ui: &mut egui::Ui, border: egui::Stroke, label: &str, value: String) {
    let text = format!("{}: {}", label, value);
    
    let frame = brutalist_frame(BrutalColors::BLACK, border)
        .inner_margin(egui::Margin::same(0));
    
    let desired_size = egui::vec2(BrutalSizes::BADGE_STAT_WIDTH, BrutalSizes::BADGE_HEIGHT);
    
    frame.show(ui, |ui| {
        ui.set_min_size(desired_size);
        ui.centered_and_justified(|ui| {
            ui.label(
                egui::RichText::new(text)
                    .color(BrutalColors::TEXT_WHITE)
                    .strong()
                    .size(BrutalSizes::BODY),
            );
        });
    });

    ui.add_space(BrutalSizes::MARGIN_MEDIUM);
}

/// Card container widget
pub fn card(ui: &mut egui::Ui, border: egui::Stroke, fill: egui::Color32, title: &str, body: impl FnOnce(&mut egui::Ui)) {
    brutalist_frame(fill, border)
        .inner_margin(egui::Margin::same(BrutalSizes::MARGIN_MEDIUM as i8))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(title)
                    .font(egui::FontId::new(BrutalSizes::TITLE, egui::FontFamily::Monospace))
                        .strong()
                        .size(BrutalSizes::TITLE)
                        .color(BrutalColors::TEXT_WHITE),
                );
            });
            ui.add_space(BrutalSizes::MARGIN_CONTENT);
            body(ui);
        });
}

/// Statistics box widget  
pub fn stat_box(ui: &mut egui::Ui, border: egui::Stroke, fill: egui::Color32, label: &str, value: String) {
    brutalist_frame(fill, border)
        .inner_margin(egui::Margin::same(BrutalSizes::MARGIN_CONTENT as i8))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(label)
                    .color(BrutalColors::TEXT_SECONDARY)
                    .size(BrutalSizes::SMALL),
            );
            ui.label(
                egui::RichText::new(value)
                    .size(BrutalSizes::STAT_VALUE)
                    .strong()
                    .color(BrutalColors::ORANGE),
            );
        });
}

/// Simple list item widget
pub fn list_item(ui: &mut egui::Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .color(egui::Color32::from_gray(230))
            .size(BrutalSizes::SMALL),
    );
}

/// Checkbox row widget
pub fn checkbox_row(ui: &mut egui::Ui, checked: bool, label: &str) {
    let glyph = if checked { "☑" } else { "☐" };
    ui.label(
        egui::RichText::new(format!("{}  {}", glyph, label))
            .color(BrutalColors::TEXT_SECONDARY)
            .size(BrutalSizes::SMALL)
            .family(egui::FontFamily::Monospace),
    );
}

/// Brutalist progress bar
pub fn progress_bar_brutal(ui: &mut egui::Ui, value_0_to_1: f32, _border: egui::Stroke) {
    let progress_bar = egui::ProgressBar::new(value_0_to_1)
        .desired_height(BrutalSizes::PROGRESS_HEIGHT)
        .fill(BrutalColors::ORANGE)
        .animate(false);
    
    ui.add(progress_bar);
}

/// Navigation button widget
pub fn nav_button(
    ui: &mut egui::Ui,
    border: egui::Stroke,
    current_nav: &mut crate::Nav,
    target_nav: crate::Nav,
    icon: Icon,
    label: &str,
) {
    let selected = *current_nav == target_nav;
    
    let bg_color = if selected {
        BrutalColors::WIDGET_SELECTED
    } else {
        BrutalColors::WIDGET_UNSELECTED
    };
    
    let text_color = if selected {
        BrutalColors::ORANGE
    } else {
        BrutalColors::TEXT_WEAK
    };

    // Create the button content
    let icon_glyph = icon_glyph(icon);
    
    let button = egui::Button::new(
        egui::RichText::new("")  // We'll use a custom draw approach
    )
    .fill(bg_color)
    .stroke(if selected { border } else { BrutalStrokes::NONE })
    .min_size(egui::vec2(BrutalSizes::NAV_BUTTON_WIDTH, BrutalSizes::NAV_BUTTON_HEIGHT))
    .corner_radius(egui::CornerRadius::ZERO);

    let response = ui.add(button);
    
    // Draw custom content on top
    let rect = response.rect;
    let icon_pos = egui::pos2(rect.min.x + BrutalSizes::MARGIN_SMALL, rect.center().y);
    let text_pos = egui::pos2(rect.min.x + BrutalSizes::MARGIN_SMALL * 3.0, rect.center().y);
    
    // Draw icon with IconFont
    ui.painter().text(
        icon_pos,
        egui::Align2::LEFT_CENTER,
        &icon_glyph,
        egui::FontId::new(BrutalSizes::SMALL, egui::FontFamily::Name("IconFont".into())),
        text_color,
    );
    
    // Draw text with Monospace font
    ui.painter().text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::new(BrutalSizes::SMALL, egui::FontFamily::Monospace),
        text_color,
    );

    if response.clicked() {
        *current_nav = target_nav;
    }
}

/// Top navigation button widget (smaller)
pub fn top_nav_button(
    ui: &mut egui::Ui, 
    border: egui::Stroke,
    current_nav: &mut crate::Nav, 
    target_nav: crate::Nav,
    icon: Icon,
    label: &str
) {
    let selected = *current_nav == target_nav;
    
    let bg_color = if selected {
        BrutalColors::WIDGET_SELECTED
    } else {
        BrutalColors::WIDGET_UNSELECTED
    };
    
    let text_color = if selected {
        BrutalColors::ORANGE
    } else {
        BrutalColors::TEXT_WEAK
    };
    
    let button = egui::Button::new("")
        .fill(bg_color)
        .stroke(if selected { border } else { BrutalStrokes::NONE })
        .min_size(egui::vec2(BrutalSizes::BUTTON_MIN_WIDTH, BrutalSizes::BUTTON_MIN_HEIGHT))
        .corner_radius(egui::CornerRadius::ZERO);

    let response = ui.add(button);
    
    // Draw custom content on top
    let rect = response.rect;
    let icon_glyph = icon_glyph(icon);
    let icon_pos = egui::pos2(rect.min.x + BrutalSizes::MARGIN_SMALL / 2.0, rect.center().y);
    let text_pos = egui::pos2(rect.min.x + BrutalSizes::MARGIN_SMALL * 2.0, rect.center().y);
    
    // Draw icon with IconFont
    ui.painter().text(
        icon_pos,
        egui::Align2::LEFT_CENTER,
        &icon_glyph,
        egui::FontId::new(BrutalSizes::TINY, egui::FontFamily::Name("IconFont".into())),
        text_color,
    );
    
    // Draw text with Monospace font
    ui.painter().text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::new(BrutalSizes::TINY, egui::FontFamily::Monospace),
        text_color,
    );

    if response.clicked() {
        *current_nav = target_nav;
    }
}

/// Icon with text widget
pub fn icon_text(ui: &mut egui::Ui, icon: Icon, text: &str, color: egui::Color32) {
    ui.horizontal(|ui| {
        // Display icon using IconFont
        ui.label(
            egui::RichText::new(&icon_glyph(icon))
                .color(color)
                .size(BrutalSizes::TITLE)
                .family(egui::FontFamily::Name("IconFont".into()))
        );
        ui.add_space(BrutalSizes::MARGIN_SMALL);
        // Display text using Brutal font
        ui.label(
            egui::RichText::new(text)
                .color(color)
                .size(BrutalSizes::TITLE)
                .font(egui::FontId::new(BrutalSizes::TITLE, egui::FontFamily::Name("Brutal".into())))
        );
    });
}

/// Status text widget
pub fn status_text(ui: &mut egui::Ui, online: bool) -> egui::Color32 {
    let (text, color) = if online {
        ("●ONLINE", BrutalColors::GREEN)
    } else {
        ("●OFFLINE", BrutalColors::RED)
    };

    ui.label(
        egui::RichText::new(text)
            .font(egui::FontId::new(BrutalSizes::SMALL, egui::FontFamily::Monospace))
            .color(color),
    );
    
    color
}

/// Time display widget
pub fn time_display(ui: &mut egui::Ui, time: &str) {
    ui.label(
        egui::RichText::new(&format!("TIME: {}", time))
            .color(BrutalColors::TEXT_WHITE)
            .size(BrutalSizes::SMALL)
            .family(egui::FontFamily::Monospace),
    );
}