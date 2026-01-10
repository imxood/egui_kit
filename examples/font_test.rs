//! Simple font test for Chinese characters
//! 简单的中文字体测试

use eframe::egui;
use egui_kit::{setup_theme, ThemeName};

#[cfg(feature = "icons")]
use egui_phosphor::regular::{HOUSE, GEAR, USER, FOLDER, FILE, ARROW_LEFT, ARROW_RIGHT, TRASH, PENCIL};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("中文字体测试"),
        ..Default::default()
    };

    eframe::run_native(
        "Font Test",
        native_options,
        Box::new(|cc| {
            // Apply theme
            setup_theme(&cc.egui_ctx, ThemeName::ModernDark);

            // Initialize font manager
            let font_manager = match egui_kit::utils::font::FontManager::new(&cc.egui_ctx) {
                Ok(manager) => {
                    println!("Font loaded: {}", manager.current_font());
                    println!("Language: {}", manager.current_language());
                    Some(manager)
                }
                Err(e) => {
                    eprintln!("Font loading failed: {}", e);
                    None
                }
            };

            #[cfg(feature = "icons")]
            println!("Icons feature enabled - Phosphor icons should render correctly");

            #[cfg(not(feature = "icons"))]
            println!("Icons feature NOT enabled - icons will show as boxes");

            Ok(Box::new(FontTestApp { font_manager }))
        }),
    )
}

struct FontTestApp {
    font_manager: Option<egui_kit::utils::font::FontManager>,
}

impl eframe::App for FontTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔤 字体测试 Font Test");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("当前字体 Current Font:");
                if let Some(manager) = &self.font_manager {
                    ui.strong(manager.current_font());
                } else {
                    ui.strong("默认 Default");
                }
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.heading("中文字符测试");
                ui.label("中文: 天地玄黄，宇宙洪荒。日月盈昃，辰宿列张。");
                ui.label("中文简体: 你好世界，欢迎使用字体测试程序！");
                ui.label("中文繁體: 您好世界，歡迎使用字體測試程式！");
                ui.label("数字: 1234567890");
                ui.label("标点: ！@#￥%……&*（）——+");
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("多语言测试");
                ui.label("English: The quick brown fox jumps over the lazy dog.");
                ui.label("日本語: いろはにほへと ちりぬるを わかよたれそ");
                ui.label("한국어: 가나다라마바사 아자차카타파하");
                ui.label("混合: Hello 世界！こんにちは 안녕하세요");
            });

            ui.add_space(10.0);

            // 图标测试区域
            #[cfg(feature = "icons")]
            ui.group(|ui| {
                ui.heading("Phosphor Icons Test");
                ui.label("Icons should render as symbols, not boxes:");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label(format!("{} House", HOUSE));
                    ui.label(format!("{} Gear", GEAR));
                    ui.label(format!("{} User", USER));
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(format!("{} Folder", FOLDER));
                    ui.label(format!("{} File", FILE));
                    ui.label(format!("{} Trash", TRASH));
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(format!("{} Arrow Left", ARROW_LEFT));
                    ui.label(format!("{} Arrow Right", ARROW_RIGHT));
                    ui.label(format!("{} Pencil", PENCIL));
                });

                ui.add_space(10.0);
                ui.label("If icons show as boxes [ ], fonts are not loaded correctly.");
            });

            #[cfg(not(feature = "icons"))]
            ui.group(|ui| {
                ui.heading("Phosphor Icons Test");
                ui.colored_label(egui::Color32::from_rgb(255, 100, 100), "Icons feature NOT enabled!");
                ui.label("Run with: cargo run --example font_test --features \"icons font\"");
            });

            ui.add_space(10.0);

            if let Some(manager) = &mut self.font_manager {
                ui.group(|ui| {
                    ui.heading("语言切换 Language Switch");

                    let languages = [
                        egui_kit::utils::font::Language::Chinese,
                        egui_kit::utils::font::Language::English,
                        egui_kit::utils::font::Language::Japanese,
                        egui_kit::utils::font::Language::Korean,
                    ];

                    for lang in languages {
                        let is_current = manager.current_language() == lang;
                        if ui
                            .selectable_label(is_current, format!("{}", lang))
                            .clicked()
                        {
                            if let Err(e) = manager.switch_language(ctx, lang) {
                                eprintln!("Failed to switch language: {}", e);
                            }
                        }
                    }
                });
            }
        });
    }
}