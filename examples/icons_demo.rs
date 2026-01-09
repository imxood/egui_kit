//! Icons Demo - Remix Icon usage example
//!
//! Run with: cargo run --example icons_demo

use eframe::egui;
use egui_remixicon::{icons, add_to_fonts};

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
            ui.heading("Remix Icon Demo");
            ui.add_space(10.0);

            // Search bar with icon
            // 带图标的搜索栏
            ui.horizontal(|ui| {
                ui.label(icons::SEARCH_LINE);
                ui.add(
                    egui::TextEdit::singleline(&mut self.search_text)
                        .hint_text("Search...")
                        .desired_width(200.0),
                );
                if ui.button(format!("{} Clear", icons::CLOSE_LINE)).clicked() {
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
                    (icons::HOME_LINE, "Home"),
                    (icons::SETTINGS_LINE, "Settings"),
                    (icons::USER_LINE, "Profile"),
                    (icons::NOTIFICATION_LINE, "Notifications"),
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
                        (icons::HOME_LINE, "HOME_LINE"),
                        (icons::ARROW_LEFT_LINE, "ARROW_LEFT_LINE"),
                        (icons::ARROW_RIGHT_LINE, "ARROW_RIGHT_LINE"),
                        (icons::ARROW_UP_LINE, "ARROW_UP_LINE"),
                        (icons::ARROW_DOWN_LINE, "ARROW_DOWN_LINE"),
                        (icons::MENU_LINE, "MENU_LINE"),
                        (icons::MORE_LINE, "MORE_LINE"),
                    ]);
                });

                ui.collapsing("Action Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::ADD_LINE, "ADD_LINE"),
                        (icons::SUBTRACT_LINE, "SUBTRACT_LINE"),
                        (icons::CLOSE_LINE, "CLOSE_LINE"),
                        (icons::CHECK_LINE, "CHECK_LINE"),
                        (icons::EDIT_LINE, "EDIT_LINE"),
                        (icons::DELETE_BIN_LINE, "DELETE_BIN_LINE"),
                        (icons::SAVE_LINE, "SAVE_LINE"),
                        (icons::DOWNLOAD_LINE, "DOWNLOAD_LINE"),
                        (icons::UPLOAD_LINE, "UPLOAD_LINE"),
                        (icons::REFRESH_LINE, "REFRESH_LINE"),
                    ]);
                });

                ui.collapsing("Communication Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::MAIL_LINE, "MAIL_LINE"),
                        (icons::MESSAGE_LINE, "MESSAGE_LINE"),
                        (icons::CHAT_1_LINE, "CHAT_1_LINE"),
                        (icons::NOTIFICATION_LINE, "NOTIFICATION_LINE"),
                        (icons::PHONE_LINE, "PHONE_LINE"),
                    ]);
                });

                ui.collapsing("Media Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::PLAY_LINE, "PLAY_LINE"),
                        (icons::PAUSE_LINE, "PAUSE_LINE"),
                        (icons::STOP_LINE, "STOP_LINE"),
                        (icons::SKIP_BACK_LINE, "SKIP_BACK_LINE"),
                        (icons::SKIP_FORWARD_LINE, "SKIP_FORWARD_LINE"),
                        (icons::VOLUME_UP_LINE, "VOLUME_UP_LINE"),
                        (icons::VOLUME_DOWN_LINE, "VOLUME_DOWN_LINE"),
                        (icons::VOLUME_MUTE_LINE, "VOLUME_MUTE_LINE"),
                    ]);
                });

                ui.collapsing("File Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::FILE_LINE, "FILE_LINE"),
                        (icons::FOLDER_LINE, "FOLDER_LINE"),
                        (icons::FOLDER_OPEN_LINE, "FOLDER_OPEN_LINE"),
                        (icons::FILE_COPY_LINE, "FILE_COPY_LINE"),
                        (icons::FILE_ADD_LINE, "FILE_ADD_LINE"),
                    ]);
                });

                ui.collapsing("System Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::SETTINGS_LINE, "SETTINGS_LINE"),
                        (icons::LOCK_LINE, "LOCK_LINE"),
                        (icons::LOCK_UNLOCK_LINE, "LOCK_UNLOCK_LINE"),
                        (icons::EYE_LINE, "EYE_LINE"),
                        (icons::EYE_OFF_LINE, "EYE_OFF_LINE"),
                        (icons::SEARCH_LINE, "SEARCH_LINE"),
                        (icons::FILTER_LINE, "FILTER_LINE"),
                        (icons::TIME_LINE, "TIME_LINE"),
                        (icons::CALENDAR_LINE, "CALENDAR_LINE"),
                    ]);
                });

                ui.collapsing("Status Icons", |ui| {
                    icon_grid(ui, &[
                        (icons::CHECKBOX_CIRCLE_LINE, "CHECKBOX_CIRCLE_LINE"),
                        (icons::CLOSE_CIRCLE_LINE, "CLOSE_CIRCLE_LINE"),
                        (icons::ERROR_WARNING_LINE, "ERROR_WARNING_LINE"),
                        (icons::INFORMATION_LINE, "INFORMATION_LINE"),
                        (icons::QUESTION_LINE, "QUESTION_LINE"),
                    ]);
                });

                ui.add_space(20.0);

                // Usage examples
                // 使用示例
                ui.heading("Usage Examples");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button(format!("{} Save", icons::SAVE_LINE)).clicked() {
                        // Save action
                    }
                    if ui.button(format!("{} Cancel", icons::CLOSE_LINE)).clicked() {
                        // Cancel action
                    }
                    if ui.button(format!("{} Delete", icons::DELETE_BIN_LINE)).clicked() {
                        // Delete action
                    }
                });

                ui.add_space(10.0);

                // Icon sizes
                // 图标大小
                ui.label("Icon Sizes:");
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(icons::STAR_LINE).size(12.0));
                    ui.label("12px");
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(icons::STAR_LINE).size(16.0));
                    ui.label("16px");
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(icons::STAR_LINE).size(24.0));
                    ui.label("24px");
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(icons::STAR_LINE).size(32.0));
                    ui.label("32px");
                });

                ui.add_space(10.0);

                // Icon colors
                // 图标颜色
                ui.label("Icon Colors:");
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(icons::HEART_FILL).size(24.0).color(egui::Color32::RED));
                    ui.label(egui::RichText::new(icons::STAR_FILL).size(24.0).color(egui::Color32::GOLD));
                    ui.label(egui::RichText::new(icons::CHECKBOX_CIRCLE_FILL).size(24.0).color(egui::Color32::GREEN));
                    ui.label(egui::RichText::new(icons::INFORMATION_FILL).size(24.0).color(egui::Color32::from_rgb(100, 149, 237)));
                    ui.label(egui::RichText::new(icons::ERROR_WARNING_FILL).size(24.0).color(egui::Color32::from_rgb(255, 165, 0)));
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
