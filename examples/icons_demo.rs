//! Icons Demo - egui-phosphor usage example
//!
//! Run with: cargo run --example icons_demo

use eframe::egui;
use egui_phosphor::{icons, add_to_fonts};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_title("Icons Demo - egui_kit"),
        ..Default::default()
    };

    eframe::run_native(
        "Icons Demo",
        options,
        Box::new(|cc| {
            // Setup fonts with icon support
            // 设置字体并支持图标
            let mut fonts = egui::FontDefinitions::default();
            add_to_fonts(&mut fonts);
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(IconsDemo::default()))
        }),
    )
}

#[derive(Default)]
struct IconsDemo {
    search_text: String,
    selected_tab: usize,
}

impl eframe::App for IconsDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Phosphor Icons Demo");
            ui.add_space(10.0);

            // Search bar with icon
            // 带图标的搜索栏
            ui.horizontal(|ui| {
                ui.label(icons::MAGNIFYING_GLASS);
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_text)
                        .hint_text("Search...")
                        .desired_width(200.0),
                );
                if ui.button(format!("{} Clear", icons::X)).clicked() {
                    self.search_text.clear();
                }
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            // Tab bar with icons
            // 带图标的标签栏
            ui.horizontal(|ui| {
                let tabs = [
                    (icons::HOUSE, "Home"),
                    (icons::GEAR, "Settings"),
                    (icons::USER, "Profile"),
                    (icons::BELL, "Notifications"),
                ];

                for (i, (icon, label)) in tabs.iter().enumerate() {
                    let text = format!("{} {}", icon, label);
                    if ui.selectable_label(self.selected_tab == i, text).clicked() {
                        self.selected_tab = i;
                    }
                }
            });

            ui.add_space(20.0);

            // Icon categories showcase
            // 图标分类展示
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.collapsing("Navigation Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::HOUSE, "HOUSE"),
                        (icons::ARROW_LEFT, "ARROW_LEFT"),
                        (icons::ARROW_RIGHT, "ARROW_RIGHT"),
                        (icons::ARROW_UP, "ARROW_UP"),
                        (icons::ARROW_DOWN, "ARROW_DOWN"),
                        (icons::LIST, "LIST"),
                        (icons::DOTS_THREE, "DOTS_THREE"),
                    ]);
                });

                ui.collapsing("Action Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::PLUS, "PLUS"),
                        (icons::MINUS, "MINUS"),
                        (icons::X, "X"),
                        (icons::CHECK, "CHECK"),
                        (icons::PENCIL, "PENCIL"),
                        (icons::TRASH, "TRASH"),
                        (icons::FLOPPY_DISK, "FLOPPY_DISK"),
                        (icons::DOWNLOAD_SIMPLE, "DOWNLOAD_SIMPLE"),
                        (icons::UPLOAD_SIMPLE, "UPLOAD_SIMPLE"),
                        (icons::ARROW_CLOCKWISE, "ARROW_CLOCKWISE"),
                    ]);
                });

                ui.collapsing("Communication Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::ENVELOPE, "ENVELOPE"),
                        (icons::CHAT_CIRCLE, "CHAT_CIRCLE"),
                        (icons::CHATS, "CHATS"),
                        (icons::BELL, "BELL"),
                        (icons::PHONE, "PHONE"),
                    ]);
                });

                ui.collapsing("Media Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::PLAY, "PLAY"),
                        (icons::PAUSE, "PAUSE"),
                        (icons::STOP, "STOP"),
                        (icons::SKIP_BACK, "SKIP_BACK"),
                        (icons::SKIP_FORWARD, "SKIP_FORWARD"),
                        (icons::VOLUME_HIGH, "VOLUME_HIGH"),
                        (icons::VOLUME_LOW, "VOLUME_LOW"),
                        (icons::VOLUME_X, "VOLUME_X"),
                    ]);
                });

                ui.collapsing("File Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::FILE, "FILE"),
                        (icons::FOLDER, "FOLDER"),
                        (icons::FOLDER_OPEN, "FOLDER_OPEN"),
                        (icons::FILE_COPY, "FILE_COPY"),
                        (icons::FILE_PLUS, "FILE_PLUS"),
                    ]);
                });

                ui.collapsing("System Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::GEAR, "GEAR"),
                        (icons::LOCK, "LOCK"),
                        (icons::LOCK_OPEN, "LOCK_OPEN"),
                        (icons::EYE, "EYE"),
                        (icons::EYE_SLASH, "EYE_SLASH"),
                        (icons::MAGNIFYING_GLASS, "MAGNIFYING_GLASS"),
                        (icons::FUNNEL, "FUNNEL"),
                        (icons::CLOCK, "CLOCK"),
                        (icons::CALENDAR, "CALENDAR"),
                    ]);
                });

                ui.collapsing("Status Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::CHECK_CIRCLE, "CHECK_CIRCLE"),
                        (icons::X_CIRCLE, "X_CIRCLE"),
                        (icons::WARNING, "WARNING"),
                        (icons::INFO, "INFO"),
                        (icons::QUESTION, "QUESTION"),
                    ]);
                });

                ui.add_space(20.0);

                // Usage examples
                // 使用示例
                ui.heading("Usage Examples");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button(format!("{} Save", icons::FLOPPY_DISK)).clicked() {
                        // Save action
                    }
                    if ui.button(format!("{} Cancel", icons::X)).clicked() {
                        // Cancel action
                    }
                    if ui.button(format!("{} Delete", icons::TRASH)).clicked() {
                        // Delete action
                    }
                });

                ui.add_space(10.0);

                // Icon sizes
                // 图标大小
                ui.label("Icon Sizes:");
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(icons::STAR).size(12.0));
                    ui.label("12px");
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(icons::STAR).size(16.0));
                    ui.label("16px");
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(icons::STAR).size(24.0));
                    ui.label("24px");
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(icons::STAR).size(32.0));
                    ui.label("32px");
                });

                ui.add_space(10.0);

                // Icon colors
                // 图标颜色
                ui.label("Icon Colors:");
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(icons::HEART).size(24.0).color(egui::Color32::RED));
                    ui.label(egui::RichText::new(icons::STAR).size(24.0).color(egui::Color32::GOLD));
                    ui.label(egui::RichText::new(icons::CHECK_CIRCLE).size(24.0).color(egui::Color32::GREEN));
                    ui.label(egui::RichText::new(icons::INFO).size(24.0).color(egui::Color32::from_rgb(100, 149, 237)));
                    ui.label(egui::RichText::new(icons::WARNING).size(24.0).color(egui::Color32::from_rgb(255, 165, 0)));
                });
            });
        });
    }
}

/// Display icons in a grid layout
/// 以网格布局显示图标
fn icon_grid(ui: &mut egui::Ui, icons_list: &[(&str, &str)]) {
    egui::Grid::new(ui.next_auto_id())
        .num_columns(4)
        .spacing([20.0, 10.0])
        .show(ui, |ui| {
            for (i, (icon, name)) in icons_list.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(*icon).size(20.0));
                    ui.label(*name);
                });
                if (i + 1) % 4 == 0 {
                    ui.end_row();
                }
            }
        });
}
