use eframe::egui;
use crate::theme::{BrutalColors, BrutalSizes, BrutalStrokes, brutalist_frame};
use crate::widgets::{Icon, badge, badge_stat, card, stat_box, list_item, checkbox_row, progress_bar_brutal, top_nav_button, nav_button, status_text, time_display, icon_text};
use crate::{App, Nav};

/// Render the main dashboard screen
pub fn render_dashboard(app: &mut App, ctx: &egui::Context) {
    let panel_bg = BrutalColors::PANEL_BG;
    let _card_bg = BrutalColors::CARD_BG;
    let border: egui::Stroke = BrutalStrokes::main();
    
    // Top bar
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        render_top_bar(app, ui);
    });

    // Sidebar
    egui::SidePanel::left("sidebar")
        .resizable(false)
        .min_width(220.0)
        .frame(brutalist_frame(BrutalColors::BLACK, BrutalStrokes::NONE).inner_margin(egui::Margin::same(12)))
        .show(ctx, |ui| {
            render_sidebar(app, ui, border);
        });

    // Main content
    egui::CentralPanel::default()
        .frame(brutalist_frame(panel_bg, BrutalStrokes::NONE).inner_margin(egui::Margin::same(BrutalSizes::MARGIN_SECTION as i8)))
        .show(ctx, |ui| {
            match app.nav {
                Nav::Dashboard => render_main_dashboard(app, ui),
                Nav::Reports => render_reports(app, ui),
                Nav::Data => render_data(app, ui),
                Nav::Tasks => render_tasks(app, ui),
                Nav::Settings => render_settings(app, ui),
            }
        });
}

fn render_top_bar(app: &mut App, ui: &mut egui::Ui) {
    let border = BrutalStrokes::main();
    
    egui::Frame::NONE
        .fill(BrutalColors::BLACK)
        .stroke(egui::Stroke::new(0.0, BrutalColors::BLACK))
        .inner_margin(egui::Margin::same(BrutalSizes::MARGIN_LARGE as i8))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Title with icon
                icon_text(ui, crate::widgets::Icon::Home, "BRUTAL INTERNAL", BrutalColors::ORANGE);
                
                ui.add_space(BrutalSizes::MARGIN_HUGE);
                
                // Navigation buttons
                ui.horizontal(|ui| {
                    top_nav_button(ui, border, &mut app.nav, Nav::Dashboard, Icon::Home, "DASHBOARD");
                    top_nav_button(ui, border, &mut app.nav, Nav::Reports, Icon::Chart, "REPORTS");
                    top_nav_button(ui, border, &mut app.nav, Nav::Data, Icon::Database, "DATA");
                    top_nav_button(ui, border, &mut app.nav, Nav::Tasks, Icon::Check, "TASKS");
                    top_nav_button(ui, border, &mut app.nav, Nav::Settings, Icon::Gear, "SETTINGS");
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Status indicator
                    status_text(ui, app.status_online);
                    ui.separator();
                    
                    // Time display
                    let time = chrono::Local::now().format("%H:%M:%S").to_string();
                    time_display(ui, &time);
                });
            });
        });
}

fn render_sidebar(app: &mut App, ui: &mut egui::Ui, border: egui::Stroke) {
    ui.vertical(|ui| {
        ui.add_space(10.0);

        // Navigation section
        ui.label(
            egui::RichText::new("NAVIGATION")
                .color(BrutalColors::TEXT_WEAK)
                .size(BrutalSizes::TINY + 1.0)
                .family(egui::FontFamily::Monospace),
        );
        
        ui.add_space(BrutalSizes::MARGIN_SMALL);

        let nav_items = [
            (Nav::Dashboard, Icon::Home, "DASHBOARD"),
            (Nav::Reports, Icon::Chart, "REPORTS"),
            (Nav::Data, Icon::Database, "DATA"),
            (Nav::Tasks, Icon::Check, "TASKS"),
            (Nav::Settings, Icon::Gear, "SETTINGS"),
        ];

        for (nav, icon, label) in nav_items {
            nav_button(ui, border, &mut app.nav, nav, icon, label);
            ui.add_space(4.0);
        }

        ui.add_space(BrutalSizes::MARGIN_LARGE);
        
        // Stats section
        ui.separator();
        ui.add_space(12.0);
        
        ui.label(
            egui::RichText::new("SYSTEM STATUS")
                .color(BrutalColors::TEXT_WEAK)
                .size(BrutalSizes::TINY + 1.0)
                .family(egui::FontFamily::Monospace),
        );
        
        ui.add_space(BrutalSizes::MARGIN_SMALL);
        
        ui.label(
            egui::RichText::new(format!("ACTIVE: {}", app.active_users))
                .color(BrutalColors::TEXT_WHITE)
                .size(BrutalSizes::TINY)
                .family(egui::FontFamily::Monospace),
        );
        
        ui.label(
            egui::RichText::new(format!("PENDING: {}", app.pending_tasks))
                .color(BrutalColors::TEXT_WHITE)
                .size(BrutalSizes::TINY)
                .family(egui::FontFamily::Monospace),
        );
    });
}

fn render_main_dashboard(app: &App, ui: &mut egui::Ui) {
    let border = BrutalStrokes::main();
    let card_bg = BrutalColors::CARD_BG;
    
    // Header badges row
    ui.horizontal(|ui| {
        badge(ui, border, app.status_online);
        badge_stat(ui, border, "ACTIVE USERS", app.active_users.to_string());
        badge_stat(ui, border, "PENDING TASKS", app.pending_tasks.to_string());
    });

    ui.add_space(BrutalSizes::MARGIN_MEDIUM);

    // Grid
    egui::Grid::new("dash_grid")
        .spacing(egui::vec2(14.0, 14.0))
        .show(ui, |ui| {
            // Left column
            ui.vertical(|ui| {
                card(ui, border, card_bg, "OVERVIEW", |ui| {
                    ui.horizontal(|ui| {
                        stat_box(ui, border, card_bg, "REVENUE", format!("€{}", app.revenue));
                        ui.add_space(10.0);
                        stat_box(ui, border, card_bg, "NEW ORDERS", app.new_orders.to_string());
                    });
                });

                ui.add_space(14.0);

                card(ui, border, card_bg, "LATEST UPDATES", |ui| {
                    list_item(ui, "◇ System check complete");
                    list_item(ui, "◇ New report generated");
                    list_item(ui, "◇ Backup successful");
                });

                ui.add_space(BrutalSizes::MARGIN_SECTION);

                card(ui, border, card_bg, "SERVER LOAD", |ui| {
                    ui.label(
                        egui::RichText::new("CPU:  53%")
                            .color(BrutalColors::TEXT_WHITE)
                            .size(BrutalSizes::SMALL),
                    );
                    progress_bar_brutal(ui, 0.53, border);
                    ui.add_space(BrutalSizes::MARGIN_SMALL);
                    ui.label(
                        egui::RichText::new("MEM:  68%")
                            .color(BrutalColors::TEXT_WHITE)
                            .size(BrutalSizes::SMALL),
                    );
                    progress_bar_brutal(ui, 0.68, border);
                });
            });

            ui.end_row();

            // Right column
            ui.vertical(|ui| {
                card(ui, border, card_bg, "TRAFFIC STATS", |ui| {
                    ui.label(
                        egui::RichText::new("VISITS LAST WEEK (placeholder)")
                            .color(BrutalColors::TEXT_SECONDARY)
                            .size(BrutalSizes::SMALL),
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("▁▂▃▂▄▅▆▆")
                                .size(BrutalSizes::TITLE_LARGE)
                                .color(BrutalColors::ORANGE),
                        )
                        .selectable(false),
                    );
                });

                ui.add_space(BrutalSizes::MARGIN_SECTION);

                card(ui, border, card_bg, "TASK LIST", |ui| {
                    checkbox_row(ui, true, "[ ] Fix Bug #104");
                    checkbox_row(ui, false, "[x] Update Database");
                    checkbox_row(ui, false, "[*] Review Logs");
                });

                ui.add_space(BrutalSizes::MARGIN_SECTION);

                card(ui, border, card_bg, "NOTIFICATIONS", |ui| {
                    list_item(ui, "! Alert: Disk space low");
                    list_item(ui, "* Reminder: Meeting at 3 PM");
                });
            });
        });
}

fn render_reports(app: &App, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.add_space(BrutalSizes::MARGIN_HUGE);

        ui.label(
            egui::RichText::new("REPORTS MODULE")
                .font(egui::FontId::new(BrutalSizes::TITLE_LARGE, egui::FontFamily::Name("Brutal".into())))
                .color(BrutalColors::TEXT_WHITE),
        );

        ui.add_space(BrutalSizes::MARGIN_HUGE);

        ui.label(
            egui::RichText::new("FINANCIAL REPORTS")
                .font(egui::FontId::new(BrutalSizes::TITLE, egui::FontFamily::Monospace))
                .color(BrutalColors::TEXT_SECONDARY),
        );

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(format!("TOTAL REVENUE: €{}", app.revenue))
                        .font(egui::FontId::new(BrutalSizes::BODY, egui::FontFamily::Monospace))
                        .color(BrutalColors::TEXT_WHITE),
                );
                ui.label(
                    egui::RichText::new("TREND: +15.3% FROM LAST MONTH")
                        .font(egui::FontId::new(BrutalSizes::BODY, egui::FontFamily::Monospace))
                        .color(BrutalColors::GREEN),
                );
            });
        });
    });
}

fn render_data(app: &App, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.add_space(BrutalSizes::MARGIN_HUGE);

        ui.label(
            egui::RichText::new("DATA MANAGEMENT")
                .font(egui::FontId::new(BrutalSizes::TITLE_LARGE, egui::FontFamily::Name("Brutal".into())))
                .color(BrutalColors::TEXT_WHITE),
        );

        ui.add_space(BrutalSizes::MARGIN_HUGE);

        ui.label(
            egui::RichText::new("DATABASE STATUS")
                .font(egui::FontId::new(BrutalSizes::TITLE, egui::FontFamily::Monospace))
                .color(BrutalColors::TEXT_SECONDARY),
        );

        ui.add_space(BrutalSizes::MARGIN_CONTENT);

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(format!("ACTIVE RECORDS: {}", app.active_users * 247))
                        .font(egui::FontId::new(BrutalSizes::BODY, egui::FontFamily::Monospace))
                        .color(BrutalColors::TEXT_WHITE),
                );
                ui.label(
                    egui::RichText::new("LAST BACKUP: 2 HOURS AGO")
                        .font(egui::FontId::new(BrutalSizes::BODY, egui::FontFamily::Monospace))
                        .color(BrutalColors::GREEN),
                );
            });
        });
    });
}

fn render_tasks(app: &App, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.add_space(BrutalSizes::MARGIN_HUGE);

        ui.label(
            egui::RichText::new("TASK MANAGEMENT")
                .font(egui::FontId::new(BrutalSizes::TITLE_LARGE, egui::FontFamily::Name("Brutal".into())))
                .color(BrutalColors::TEXT_WHITE),
        );

        ui.add_space(BrutalSizes::MARGIN_HUGE);

        ui.label(
            egui::RichText::new("PENDING TASKS")
                .font(egui::FontId::new(BrutalSizes::TITLE, egui::FontFamily::Monospace))
                .color(BrutalColors::TEXT_SECONDARY),
        );

        ui.add_space(BrutalSizes::MARGIN_CONTENT);

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(format!("TOTAL PENDING: {}", app.pending_tasks))
                        .font(egui::FontId::new(BrutalSizes::BODY, egui::FontFamily::Monospace))
                        .color(BrutalColors::TEXT_WHITE),
                );
                ui.label(
                    egui::RichText::new("HIGH PRIORITY: 2")
                        .font(egui::FontId::new(BrutalSizes::BODY, egui::FontFamily::Monospace))
                        .color(BrutalColors::RED),
                );
            });
        });
    });
}

fn render_settings(_app: &App, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.add_space(BrutalSizes::MARGIN_HUGE);

        ui.label(
            egui::RichText::new("SYSTEM SETTINGS")
                .font(egui::FontId::new(BrutalSizes::TITLE_LARGE, egui::FontFamily::Name("Brutal".into())))
                .color(BrutalColors::TEXT_WHITE),
        );

        ui.add_space(BrutalSizes::MARGIN_HUGE);

        ui.label(
            egui::RichText::new("CONFIGURATION")
                .font(egui::FontId::new(BrutalSizes::TITLE, egui::FontFamily::Monospace))
                .color(BrutalColors::TEXT_SECONDARY),
        );

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("THEME: BRUTAL DARK")
                        .font(egui::FontId::new(BrutalSizes::BODY, egui::FontFamily::Monospace))
                        .color(BrutalColors::TEXT_WHITE),
                );
                ui.label(
                    egui::RichText::new(format!("VERSION: {}", env!("CARGO_PKG_VERSION")))
                        .font(egui::FontId::new(BrutalSizes::BODY, egui::FontFamily::Monospace))
                        .color(BrutalColors::TEXT_SECONDARY),
                );
            });
        });
    });
}