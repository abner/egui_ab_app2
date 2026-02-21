use eframe::egui;
use crate::theme::{BrutalColors, BrutalSizes, brutalist_frame};
use crate::{App, Screen};

// ==== SPLASH SCREEN CONFIGURATION ====
// Toggle between image formats for the splash screen
const USE_SVG_SPLASH: bool = false;  // Set to false to use PNG instead
// Note: Image paths are hardcoded in display_svg() and display_png() functions

// =====================================

// ==== ORIGINAL TEXT-BASED SPLASH (COMMENTED FOR EASY SWITCHING) ====
// Uncomment this section and comment the image splash above to use text-based splash as default

/// Original text-based startup splash screen
pub fn render_startup(app: &mut App, ctx: &egui::Context) {
    let accent_orange = egui::Color32::from_rgb(0xF9, 0x8C, 0x1F); // #f98c1f
    egui::CentralPanel::default()
        .frame(
            brutalist_frame(accent_orange, egui::Stroke::new(2.0, egui::Color32::from_gray(40)))
                .inner_margin(egui::Margin::same(24)),
        )
        .show(ctx, |ui| {
            let rect = ui.max_rect();

            // Brutalist "glitch blocks" on the left
            paint_glitch_blocks(ui.painter(), rect);

            ui.vertical_centered(|ui| {
                ui.add_space(rect.height() * 0.18);

                // Big brutal title
                let title = egui::RichText::new("BRUTAL INTERNAL")
                    .font(egui::FontId::new(84.0, egui::FontFamily::Name("Brutal".into())))
                    .color(BrutalColors::TEXT_WHITE);

                ui.label(title);

                ui.add_space(26.0);

                // Enter button
                let button = egui::Button::new(
                    egui::RichText::new("ENTER SYSTEM")
                        .color(BrutalColors::TEXT_WHITE)
                        .strong(),
                )
                .min_size(egui::vec2(220.0, 54.0))
                .fill(BrutalColors::ORANGE)
                .corner_radius(egui::CornerRadius::ZERO)
                .stroke(egui::Stroke::new(2.0, BrutalColors::TEXT_WHITE));

                if ui.add(button).clicked() {
                    app.screen = Screen::Dashboard;
                }

                ui.add_space(20.0);

                // Button to view image splash (now the default)
                let alt_button = egui::Button::new(
                    egui::RichText::new("VIEW IMAGE SPLASH")
                        .color(BrutalColors::TEXT_SECONDARY)
                        .size(14.0),
                )
                .min_size(egui::vec2(200.0, 35.0))
                .fill(BrutalColors::BLACK)
                .corner_radius(egui::CornerRadius::ZERO)
                .stroke(egui::Stroke::new(1.0, BrutalColors::TEXT_SECONDARY));

                if ui.add(alt_button).clicked() {
                    app.screen = Screen::AlternativeSplash;
                }

                ui.add_space(rect.height() * 0.22);

                ui.label(
                    egui::RichText::new("v0.1.0 • Internal Build")
                        .color(BrutalColors::TEXT_SECONDARY)
                        .size(BrutalSizes::BODY),
                );
            });
        });
}

fn paint_glitch_blocks(painter: &egui::Painter, rect: egui::Rect) {
    // Left-side brutal blocks
    let left = egui::Rect::from_min_max(rect.min, egui::pos2(rect.min.x + 140.0, rect.max.y));
    painter.rect_filled(left, egui::CornerRadius::ZERO, egui::Color32::from_black_alpha(35));

    // Hard glitch blocks with theme colors
    let blocks = [
        (egui::Rect::from_min_size(egui::pos2(rect.min.x + 18.0, rect.min.y + 80.0), egui::vec2(70.0, 18.0)), BrutalColors::BLACK),
        (egui::Rect::from_min_size(egui::pos2(rect.min.x + 10.0, rect.min.y + 140.0), egui::vec2(110.0, 26.0)), BrutalColors::WHITE),
        (egui::Rect::from_min_size(egui::pos2(rect.min.x + 28.0, rect.min.y + 220.0), egui::vec2(46.0, 16.0)), BrutalColors::BLACK),
        (egui::Rect::from_min_size(egui::pos2(rect.min.x + 12.0, rect.min.y + 300.0), egui::vec2(120.0, 20.0)), BrutalColors::WHITE),
        (egui::Rect::from_min_size(egui::pos2(rect.min.x + 18.0, rect.min.y + 360.0), egui::vec2(90.0, 18.0)), BrutalColors::BLACK),
        (egui::Rect::from_min_size(egui::pos2(rect.min.x + 20.0, rect.min.y + 420.0), egui::vec2(130.0, 34.0)), BrutalColors::WHITE),
    ];

    for (r, color) in &blocks {
        painter.rect_filled(*r, egui::CornerRadius::ZERO, *color);
    }
}

/// Main splash screen renderer (now the default screen)
pub fn render_alternative_splash(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(BrutalColors::ORANGE))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let rect = ui.max_rect();
                
                // Add some top spacing
                ui.add_space(rect.height() * 0.1);
                
                // Display image based on configuration
                if USE_SVG_SPLASH {
                    display_svg(ui, ctx, rect);
                } else {
                    display_png(ui, ctx, rect);
                }
                
                ui.add_space(40.0);
                
                // Enter button with brutalist styling
                let button = egui::Button::new(
                    egui::RichText::new("ENTER APPLICATION")
                        .color(BrutalColors::TEXT_WHITE)
                        .size(18.0)
                        .strong(),
                )
                .min_size(egui::vec2(250.0, 60.0))
                .fill(BrutalColors::ORANGE)
                .corner_radius(egui::CornerRadius::ZERO)
                .stroke(egui::Stroke::new(3.0, BrutalColors::TEXT_WHITE));

                if ui.add(button).clicked() {
                    app.screen = Screen::Dashboard;
                }
                
                ui.add_space(20.0);
                
                // Button to switch to original text-based splash
                let alt_button = egui::Button::new(
                    egui::RichText::new("VIEW TEXT SPLASH")
                        .color(BrutalColors::TEXT_SECONDARY)
                        .size(14.0),
                )
                .min_size(egui::vec2(200.0, 35.0))
                .fill(BrutalColors::BLACK)
                .corner_radius(egui::CornerRadius::ZERO)
                .stroke(egui::Stroke::new(1.0, BrutalColors::TEXT_SECONDARY));

                if ui.add(alt_button).clicked() {
                    app.screen = Screen::Startup;
                }
            });
        });
}

/// Display SVG splash screen image
fn display_svg(ui: &mut egui::Ui, ctx: &egui::Context, rect: egui::Rect) {
    let svg_bytes = include_bytes!("../assets/images/splash.svg");
    let svg_size = egui::vec2(rect.width() * 0.8, rect.height() * 0.6);
    
    let usvg_options = usvg::Options::default();
    match egui_extras::image::load_svg_bytes(svg_bytes, &usvg_options) {
        Ok(color_image) => {
            // Check if image has any non-transparent pixels
            let has_content = color_image.pixels.iter().any(|&pixel| {
                let [r, g, b, a] = pixel.to_array();
                a > 0 && (r > 0 || g > 0 || b > 0)
            });
            
            if has_content {
                let texture = ctx.load_texture("splash_svg", color_image, Default::default());
                let image = egui::Image::from_texture(&texture);
                ui.add(image.fit_to_exact_size(svg_size));
            } else {
                // Fallback for empty SVG
                display_fallback_message(ui, "SVG loaded but contains no visible content", svg_size);
            }
        }
        Err(e) => {
            display_fallback_message(ui, &format!("SVG Loading Error: {}", e), svg_size);
        }
    }
}

/// Display PNG splash screen image
fn display_png(ui: &mut egui::Ui, ctx: &egui::Context, rect: egui::Rect) {
    let png_bytes = include_bytes!("../assets/images/AB_APP_SPLASH.png");
    let png_size = egui::vec2(rect.width() * 0.8, rect.height() * 0.6);
    
    match egui_extras::image::load_image_bytes(png_bytes) {
        Ok(color_image) => {
            let texture = ctx.load_texture("splash_png", color_image, Default::default());
            let image = egui::Image::from_texture(&texture);
            ui.add(image.fit_to_exact_size(png_size));
        }
        Err(e) => {
            display_fallback_message(ui, &format!("PNG Loading Error: {}", e), png_size);
        }
    }
}

/// Display fallback message when image loading fails
fn display_fallback_message(ui: &mut egui::Ui, message: &str, size: egui::Vec2) {
    ui.allocate_exact_size(size, egui::Sense::hover());
    ui.label(
        egui::RichText::new("SPLASH IMAGE")
            .font(egui::FontId::new(48.0, egui::FontFamily::Name("Brutal".into())))
            .color(BrutalColors::TEXT_WHITE)
    );
    ui.label(
        egui::RichText::new(message)
            .color(BrutalColors::TEXT_SECONDARY)
            .size(16.0)
    );
}