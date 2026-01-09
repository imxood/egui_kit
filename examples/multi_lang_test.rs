//! Multi-language font support test
//! 多语言字体支持测试

use eframe::egui;
use egui_kit::{setup_theme, ThemeName};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_title("🌍 Multi-language Font Support 多语言字体支持"),
        ..Default::default()
    };

    eframe::run_native(
        "Multi-language Test",
        native_options,
        Box::new(|cc| {
            // Apply theme
            setup_theme(&cc.egui_ctx, ThemeName::ModernDark);

            // Initialize font manager
            let font_manager = match egui_kit::utils::font::FontManager::new(&cc.egui_ctx) {
                Ok(manager) => {
                    println!("✅ Font loaded: {}", manager.current_font());
                    Some(manager)
                }
                Err(e) => {
                    eprintln!("⚠️  Font loading failed: {}", e);
                    None
                }
            };

            Ok(Box::new(MultiLanguageTestApp {
                font_manager,
                selected_languages: vec![egui_kit::utils::font::Language::Chinese],
                is_multi_language_mode: false,
                custom_text: "Hello 世界! こんにちは 안녕하세요".to_string(),
            }))
        }),
    )
}

struct MultiLanguageTestApp {
    font_manager: Option<egui_kit::utils::font::FontManager>,
    selected_languages: Vec<egui_kit::utils::font::Language>,
    is_multi_language_mode: bool,
    custom_text: String,
}

impl MultiLanguageTestApp {
    /// Show multi-language font selector component
    /// 显示多语言字体选择器组件
    fn show_language_selector(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("🌍 多语言字体选择器 Multi-language Font Selector");
        ui.separator();

        // Mode toggle - collect state changes before applying
        let mut mode_changed = false;
        let mut new_mode = self.is_multi_language_mode;

        ui.horizontal(|ui| {
            ui.label("模式 Mode:");
            let mode_text = if new_mode { "多语言 Multi" } else { "单一 Single" };
            if ui.selectable_label(new_mode, mode_text).clicked() {
                new_mode = !new_mode;
                mode_changed = true;
            }
        });

        if mode_changed {
            self.is_multi_language_mode = new_mode;
            if let Some(manager) = &mut self.font_manager {
                manager.set_multi_language_mode(new_mode);
                self.apply_fonts(ctx);
            }
        }

        ui.add_space(10.0);

        if self.is_multi_language_mode {
            ui.group(|ui| {
                ui.heading("选择语言 Select Languages:");
                ui.separator();

                let all_languages = [
                    egui_kit::utils::font::Language::Chinese,
                    egui_kit::utils::font::Language::English,
                    egui_kit::utils::font::Language::Japanese,
                    egui_kit::utils::font::Language::Korean,
                ];

                let mut languages_changed = false;

                for language in all_languages {
                    let mut is_selected = self.selected_languages.contains(&language);
                    if ui.checkbox(&mut is_selected, format!("{}", language)).changed() {
                        languages_changed = true;
                        if is_selected {
                            self.selected_languages.push(language);
                        } else {
                            self.selected_languages.retain(|&l| l != language);
                        }
                    }
                }

                if languages_changed {
                    self.apply_fonts(ctx);
                }

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("全选 Select All").clicked() {
                        self.selected_languages = all_languages.to_vec();
                        self.apply_fonts(ctx);
                    }
                    if ui.button("清空 Clear").clicked() {
                        self.selected_languages.clear();
                        // Keep at least one language
                        self.selected_languages.push(egui_kit::utils::font::Language::English);
                        self.apply_fonts(ctx);
                    }
                });
            });
        } else {
            ui.group(|ui| {
                ui.heading("单一语言模式 Single Language Mode");
                ui.separator();
                ui.label("使用传统的单语言字体切换");
                ui.label("Use traditional single-language font switching");
            });
        }

        ui.add_space(10.0);

        // Current status
        if let Some(manager) = &self.font_manager {
            self.show_font_status(ui, manager);
        }
    }

    /// Show current font status
    /// 显示当前字体状态
    fn show_font_status(&self, ui: &mut egui::Ui, manager: &egui_kit::utils::font::FontManager) {
        ui.group(|ui| {
            ui.heading("📊 当前状态 Current Status");
            ui.separator();

            if self.is_multi_language_mode {
                ui.horizontal(|ui| {
                    ui.label("模式 Mode:");
                    ui.strong("多语言 Multi-language");
                });

                ui.horizontal(|ui| {
                    ui.label("支持语言:");
                    let (languages, fonts) = manager.multi_language_info();
                    for (i, language) in languages.iter().enumerate() {
                        if i > 0 {
                            ui.label(" → ");
                        }
                        ui.strong(format!("{}", language));
                        if i < fonts.len() {
                            ui.label(format!("({})", fonts[i]));
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("字体数量 Font Count:");
                    let (_, fonts) = manager.multi_language_info();
                    ui.strong(format!("{}", fonts.len()));
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("模式 Mode:");
                    ui.strong("单一语言 Single-language");
                });

                ui.horizontal(|ui| {
                    ui.label("当前语言 Current:");
                    ui.strong(format!("{}", manager.current_language()));
                });

                ui.horizontal(|ui| {
                    ui.label("当前字体 Current:");
                    ui.strong(manager.current_font());
                });
            }
        });
    }

    /// Apply fonts based on current mode
    /// 根据当前模式应用字体
    fn apply_fonts(&mut self, ctx: &egui::Context) {
        if let Some(manager) = &mut self.font_manager {
            // Update manager's language selection
            for &language in &self.selected_languages {
                manager.add_language(language);
            }

            if self.is_multi_language_mode {
                if let Err(e) = manager.apply_multi_language_fonts(ctx) {
                    eprintln!("Failed to apply multi-language fonts: {}", e);
                }
            } else {
                // Use first selected language for single mode
                if let Some(&language) = self.selected_languages.first() {
                    if let Err(e) = manager.switch_language(ctx, language) {
                        eprintln!("Failed to switch language: {}", e);
                    }
                }
            }
        }
    }

    /// Show multi-language text preview
    /// 显示多语言文本预览
    fn show_text_preview(&mut self, ui: &mut egui::Ui) {
        ui.heading("📝 文本预览 Text Preview");
        ui.separator();

        // Custom text input
        ui.horizontal(|ui| {
            ui.label("自定义文本:");
            ui.text_edit_singleline(&mut self.custom_text);
        });

        ui.add_space(10.0);

        // Preset texts
        ui.group(|ui| {
            ui.heading("预设文本 Preset Texts");
            ui.separator();

            let preset_texts = [
                ("中文", "你好世界！欢迎使用多语言字体支持。天地玄黄，宇宙洪荒。"),
                ("English", "Hello World! Welcome to multi-language font support. The quick brown fox jumps over the lazy dog."),
                ("日本語", "こんにちは世界！多言語フォントサポートへようこそ。いろはにほへと ちりぬるを。"),
                ("한국어", "안녕하세요 세계! 다국어 글꼴 지원에 오신 것을 환영합니다. 가나다라마바사."),
                ("混合 Mixed", "Hello 世界! こんにちは 안녕하세요. English + 中文 + 日本語 + 한국어"),
            ];

            for (title, text) in preset_texts {
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", title));
                    ui.label(text);
                });
            }
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.heading("🎯 混合语言测试 Mixed Language Test");
            ui.separator();

            let mixed_texts = [
                "English 中文 日本語 한국어",
                "Hello 世界! Welcome 欢迎! ようこそ! 환영합니다!",
                "数字 123 中文 测试 English Test 日本語テスト 한국어테스트",
                "Symbols: !@#$%^&*() 中文 English 日本語 한국어",
            ];

            for text in mixed_texts {
                ui.label(text);
            }
        });
    }
}

impl eframe::App for MultiLanguageTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌍 多语言字体支持测试 Multi-language Font Support Test");
            ui.separator();

            // Language selector (left side)
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(500.0);
                    self.show_language_selector(ui, ctx);
                });

                ui.separator();

                // Text preview (right side)
                ui.vertical(|ui| {
                    ui.set_width(450.0);
                    self.show_text_preview(ui);
                });
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.heading("💡 使用说明 Usage Guide");
                ui.separator();
                ui.label("1. 切换到 '多语言 Multi' 模式启用多语言支持");
                ui.label("1. Switch to 'Multi-language Multi' mode to enable multi-language support");
                ui.add_space(5.0);
                ui.label("2. 勾选需要支持的语言，系统会按选择顺序建立字体回退链");
                ui.label("2. Check the languages you need, system will create font fallback chain in selection order");
                ui.add_space(5.0);
                ui.label("3. 混合语言文本会自动使用合适的字体渲染每个字符");
                ui.label("3. Mixed language text will automatically use appropriate fonts for each character");
                ui.add_space(5.0);
                ui.label("4. 观察预设文本，确保所有语言字符都能正确显示");
                ui.label("4. Observe preset texts to ensure all language characters display correctly");
            });
        });
    }
}