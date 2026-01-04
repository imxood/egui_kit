//! egui Theme Showcase with Comprehensive Font Testing
//!
//! This example combines all font testing features:
//! - Basic font loading and switching
//! - Memory cleanup testing
//! - Multi-language font support
//! - Theme demonstration
//!
//! 运行方式:
//! cargo run -p egui_kit --example showcase

use eframe::egui;
use egui_kit::foundation::{ThemeConfig, ThemePreset, h1, h2, h3, h4, h5, h6, h7, setup_theme};

#[cfg(feature = "font")]
use egui_kit::utils::font::{FontManager, Language};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("🎨 egui Theme Showcase - Comprehensive Testing"),
        ..Default::default()
    };

    eframe::run_native(
        "Theme Showcase",
        native_options,
        Box::new(|cc| {
            // Apply default theme
            setup_theme(&cc.egui_ctx, ThemePreset::Dark);

            // Initialize font manager
            let font_manager = match FontManager::new(&cc.egui_ctx) {
                Ok(manager) => {
                    println!("✅ Font loaded: {}", manager.current_font());
                    println!("   Language: {}", manager.current_language());

                    // Start with Chinese selected but don't apply it yet
                    // Let the UI control the initial font state
                    Some(manager)
                }
                Err(e) => {
                    eprintln!("⚠️  Font loading failed: {}", e);
                    None
                }
            };

            Ok(Box::new(ComprehensiveShowcaseApp {
                current_theme: ThemePreset::Dark,
                slider_value: 50.0,
                text_input: "Type something here...".to_string(),
                checkbox: true,
                radio: 0,
                font_manager,

                // Font testing state - start with English only for clean testing
                selected_languages: vec![Language::English],
                is_multi_language_mode: false,
                switch_count: 0,
                last_switch_time: std::time::Instant::now(),
                custom_text: "Hello 世界! こんにちは 안녕하세요".to_string(),

                // UI state
                active_tab: Tab::Theme,
            }))
        }),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Theme,
    SingleFont,
    MultiLanguage,
    MemoryTest,
}

struct ComprehensiveShowcaseApp {
    // Original showcase state
    current_theme: ThemePreset,
    slider_value: f32,
    text_input: String,
    checkbox: bool,
    radio: usize,

    #[cfg(feature = "font")]
    font_manager: Option<FontManager>,

    // Font testing state
    #[cfg(feature = "font")]
    selected_languages: Vec<Language>,
    #[cfg(feature = "font")]
    is_multi_language_mode: bool,
    #[cfg(feature = "font")]
    switch_count: u32,
    #[cfg(feature = "font")]
    last_switch_time: std::time::Instant,
    #[cfg(feature = "font")]
    custom_text: String,

    // UI state
    active_tab: Tab,
}

impl ComprehensiveShowcaseApp {
    /// Initialize fonts to ensure clean state
    /// 初始化字体以确保清洁状态
    #[cfg(feature = "font")]
    fn initialize_fonts(&mut self, ctx: &egui::Context) {
        if let Some(manager) = &mut self.font_manager {
            // Apply initial English-only font state
            // 应用初始的仅英文字体状态
            if let Err(e) = manager.switch_language(ctx, Language::English) {
                eprintln!("Failed to initialize English font: {}", e);
            } else {
                println!("✅ Initialized with English font only");
            }
        }
    }

    /// Show tab selector
    fn show_tab_selector(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("📋 功能选择器 Feature Selector:");
            ui.separator();

            let tabs = [
                (Tab::Theme, "🎨 主题", "Theme demonstration"),
                (Tab::SingleFont, "🔤 单字体", "Single font testing"),
                (Tab::MultiLanguage, "🌍 多语言", "Multi-language support"),
                (Tab::MemoryTest, "🧹 内存测试", "Memory cleanup test"),
            ];

            for (tab, title, tooltip) in tabs {
                let is_active = self.active_tab == tab;
                if ui
                    .selectable_label(is_active, title)
                    .on_hover_text(tooltip)
                    .clicked()
                {
                    self.active_tab = tab;
                }
            }
        });
    }

    /// Show original theme showcase
    #[cfg(not(feature = "font"))]
    fn show_theme_showcase(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎨 Theme Showcase (Font loading not available)");
        ui.separator();
        ui.colored_label(
            egui::Color32::from_rgb(200, 150, 100),
            "⚠️  Font loading feature not enabled",
        );
        ui.label("Run with: --features font");
        ui.add_space(10.0);
        ui.label("Using default egui embedded fonts.");
    }

    /// Show theme showcase with font support
    #[cfg(feature = "font")]
    fn show_theme_showcase(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🎨 Theme Showcase");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Current Theme:");
            ui.strong(self.current_theme.name());
        });

        // Theme selector
        ui.horizontal_wrapped(|ui| {
            for preset in ThemePreset::all() {
                let is_selected = self.current_theme == *preset;
                if ui
                    .selectable_label(is_selected, preset.name())
                    .on_hover_text(preset.description())
                    .clicked()
                {
                    self.current_theme = *preset;
                    let theme = ThemeConfig::from_preset(*preset);
                    egui_kit::apply_theme(ctx, &theme);
                }
            }
        });

        ui.add_space(20.0);

        // Font status
        if let Some(manager) = &self.font_manager {
            ui.group(|ui| {
                ui.heading("🔤 Font Status");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Current Font:");
                    ui.strong(manager.current_font());
                    ui.label(format!("({})", manager.current_language()));
                });

                if manager.is_multi_language_mode() {
                    ui.horizontal(|ui| {
                        ui.label("Multi-language:");
                        ui.strong("ON");
                        let (languages, fonts) = manager.multi_language_info();
                        ui.label(format!(
                            "{} languages, {} fonts",
                            languages.len(),
                            fonts.len()
                        ));
                    });
                }
            });
        }

        ui.add_space(20.0);

        // Widget showcase
        self.show_widget_showcase(ui);
        self.show_heading_showcase(ui);
    }

    fn show_widget_showcase(&mut self, ui: &mut egui::Ui) {
        ui.heading("Widget Showcase");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Buttons:");
            if ui.button("Normal Button").clicked() {}
            if ui.small_button("Small").clicked() {}
            ui.add_enabled(false, egui::Button::new("Disabled"));
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Text Input:");
            ui.text_edit_singleline(&mut self.text_input);
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Checkbox:");
            ui.checkbox(&mut self.checkbox, "Enable feature");
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Radio:");
            ui.radio_value(&mut self.radio, 0, "Option A");
            ui.radio_value(&mut self.radio, 1, "Option B");
            ui.radio_value(&mut self.radio, 2, "Option C");
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Slider:");
            ui.add(egui::Slider::new(&mut self.slider_value, 0.0..=100.0));
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("ComboBox:");
            egui::ComboBox::from_id_salt("combo")
                .selected_text(format!("Option {}", self.radio + 1))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.radio, 0, "Option 1");
                    ui.selectable_value(&mut self.radio, 1, "Option 2");
                    ui.selectable_value(&mut self.radio, 2, "Option 3");
                });
        });
    }

    fn show_heading_showcase(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        ui.heading("Heading Styles (h1-h7)");
        ui.separator();

        let theme = ThemeConfig::from_preset(self.current_theme);

        ui.group(|ui| {
            ui.heading("Basic Headings:");
            ui.add_space(5.0);
            ui.add(h1("Heading 1 (32px)"));
            ui.add(h2("Heading 2 (28px)"));
            ui.add(h3("Heading 3 (24px)"));
            ui.add(h4("Heading 4 (20px)"));
            ui.add(h5("Heading 5 (16px)"));
            ui.add(h6("Heading 6 (14px)"));
            ui.add(h7("Heading 7 (12px)"));
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.heading("Colored Headings:");
            ui.add_space(5.0);
            ui.add(h2("Primary Heading").color(theme.colors.primary));
            ui.add(h3("Success Message").color(theme.colors.success));
            ui.add(h4("Warning Alert").color(theme.colors.warning));
            ui.add(h5("Error Notice").color(theme.colors.error));
            ui.add(h6("Info Text").color(theme.colors.info));
        });
    }

    /// Show single font testing
    #[cfg(feature = "font")]
    fn show_single_font_test(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🔤 Single Font Testing");
        ui.separator();

        if let Some(manager) = &mut self.font_manager {
            // Current status
            ui.group(|ui| {
                ui.heading("Current Status:");
                ui.horizontal(|ui| {
                    ui.label("Font:");
                    ui.strong(manager.current_font());
                });
                ui.horizontal(|ui| {
                    ui.label("Language:");
                    ui.strong(format!("{}", manager.current_language()));
                });
            });

            ui.add_space(10.0);

            // Language switcher
            ui.group(|ui| {
                ui.heading("Language Switch:");
                let languages = [
                    Language::Chinese,
                    Language::English,
                    Language::Japanese,
                    Language::Korean,
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

            ui.add_space(10.0);

            // Font preview
            self.show_font_preview(ui);
        } else {
            ui.colored_label(
                egui::Color32::from_rgb(200, 100, 100),
                "⚠️  Font loading not available",
            );
        }
    }

    /// Show multi-language support
    #[cfg(feature = "font")]
    fn show_multi_language_support(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🌍 Multi-language Font Support");
        ui.separator();

        // Mode toggle - collect changes before applying
        let mut mode_changed = false;
        let mut new_mode = self.is_multi_language_mode;

        ui.horizontal(|ui| {
            ui.label("Mode:");
            let mode_text = if new_mode { "Multi-language" } else { "Single" };
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
                ui.heading("Select Languages:");
                let all_languages = [
                    Language::Chinese,
                    Language::English,
                    Language::Japanese,
                    Language::Korean,
                ];

                let mut languages_changed = false;

                for language in all_languages {
                    let mut is_selected = self.selected_languages.contains(&language);
                    if ui
                        .checkbox(&mut is_selected, format!("{}", language))
                        .changed()
                    {
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
                    if ui.button("Select All").clicked() {
                        self.selected_languages = all_languages.to_vec();
                        self.apply_fonts(ctx);
                    }
                    if ui.button("Clear").clicked() {
                        self.selected_languages.clear();
                        self.selected_languages.push(Language::English);
                        self.apply_fonts(ctx);
                    }
                });
            });
        }

        ui.add_space(10.0);

        // Status - use immutable borrow
        if let Some(manager) = &self.font_manager {
            self.show_font_status(ui, manager);
        }

        ui.add_space(10.0);

        // Text preview
        self.show_text_preview(ui);
    }

    /// Show memory cleanup test
    #[cfg(feature = "font")]
    fn show_memory_test(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🧹 Memory Cleanup Test");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Switch Count:");
            ui.strong(format!("{}", self.switch_count));
            ui.label(format!(
                "({:.1}s ago)",
                self.last_switch_time.elapsed().as_secs_f32()
            ));
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.heading("Font Switching Test:");

            // Collect button clicks before applying
            let mut switch_language = None;

            ui.horizontal(|ui| {
                if ui.button("Chinese").clicked() {
                    switch_language = Some(Language::Chinese);
                }

                if ui.button("English").clicked() {
                    switch_language = Some(Language::English);
                }

                if ui.button("Japanese").clicked() {
                    switch_language = Some(Language::Japanese);
                }

                if ui.button("Korean").clicked() {
                    switch_language = Some(Language::Korean);
                }
            });

            ui.add_space(10.0);

            let mut quick_switch = false;
            let mut show_info = false;

            ui.horizontal(|ui| {
                if ui.button("🔄 Quick Switch 5 Times").clicked() {
                    quick_switch = true;
                }

                if ui.button("📊 Show Memory Info").clicked() {
                    show_info = true;
                }
            });

            // Apply the collected actions
            if let Some(language) = switch_language {
                if let Some(manager) = &mut self.font_manager {
                    if let Err(e) = manager.switch_language(ctx, language) {
                        eprintln!("Failed to switch language: {}", e);
                    } else {
                        self.increment_switch_count();
                    }
                }
            }

            if quick_switch {
                let languages = [
                    Language::Chinese,
                    Language::English,
                    Language::Japanese,
                    Language::Korean,
                    Language::Chinese,
                ];

                // Increment count first, then switch languages
                self.switch_count += languages.len() as u32;
                self.last_switch_time = std::time::Instant::now();

                if let Some(manager) = &mut self.font_manager {
                    for lang in languages {
                        if let Err(e) = manager.switch_language(ctx, lang) {
                            eprintln!("Failed to switch language: {}", e);
                        }
                    }
                }
            }

            if show_info {
                self.show_memory_info();
            }
        });

        ui.add_space(10.0);

        self.show_font_preview(ui);
    }

    // Helper methods
    #[cfg(feature = "font")]
    fn increment_switch_count(&mut self) {
        self.switch_count += 1;
        self.last_switch_time = std::time::Instant::now();
    }

    #[cfg(feature = "font")]
    fn apply_fonts(&mut self, ctx: &egui::Context) {
        if let Some(manager) = &mut self.font_manager {
            eprintln!("self.selected_languages: {:?}", self.selected_languages);

            // Use the abstracted sync method from FontManager
            // 使用FontManager中抽象的同步方法
            if let Err(e) = manager.sync_selected_languages(ctx, &self.selected_languages) {
                eprintln!("Failed to sync selected languages: {}", e);
            }
        }
    }

    #[cfg(feature = "font")]
    fn show_font_status(&self, ui: &mut egui::Ui, manager: &egui_kit::utils::font::FontManager) {
        ui.group(|ui| {
            ui.heading("📊 Status");
            ui.separator();

            if self.is_multi_language_mode {
                ui.horizontal(|ui| {
                    ui.label("Mode:");
                    ui.strong("Multi-language");
                });

                ui.horizontal(|ui| {
                    ui.label("Languages:");
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
                    ui.label("Font Count:");
                    let (_, fonts) = manager.multi_language_info();
                    ui.strong(format!("{}", fonts.len()));
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("Mode:");
                    ui.strong("Single-language");
                });

                ui.horizontal(|ui| {
                    ui.label("Current:");
                    ui.strong(format!("{}", manager.current_language()));
                });

                ui.horizontal(|ui| {
                    ui.label("Font:");
                    ui.strong(manager.current_font());
                });
            }
        });
    }

    #[cfg(feature = "font")]
    fn show_font_preview(&self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("📝 Font Preview:");
            ui.separator();
            ui.label("English: The quick brown fox jumps over the lazy dog.");
            ui.label("中文: 天地玄黄，宇宙洪荒。日月盈昃，辰宿列张。");
            ui.label("日本語: いろはにほへと ちりぬるを わかよたれそ");
            ui.label("한국어: 가나다라마바사 아자차카타파하");
            ui.label("Mixed: Hello 世界! こんにちは 안녕하세요");
            ui.label("Numbers: 0123456789");
            ui.label("Symbols: !@#$%^&*()_+-=[]{}|;:',.<>?/");
        });
    }

    #[cfg(feature = "font")]
    fn show_text_preview(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("📝 Text Preview:");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Custom:");
                ui.text_edit_singleline(&mut self.custom_text);
            });

            ui.add_space(10.0);

            let preset_texts = [
                (
                    "中文",
                    "你好世界！欢迎使用多语言字体支持。天地玄黄，宇宙洪荒。",
                ),
                (
                    "English",
                    "Hello World! Welcome to multi-language font support.",
                ),
                (
                    "日本語",
                    "こんにちは世界！多言語フォントサポートへようこそ。",
                ),
                (
                    "한국어",
                    "안녕하세요 세계! 다국어 글꼴 지원에 오신 것을 환영합니다.",
                ),
                (
                    "Mixed",
                    "Hello 世界! こんにちは 안녕하세요. English + 中文 + 日本語 + 한국어",
                ),
            ];

            for (title, text) in preset_texts {
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", title));
                    ui.label(text);
                });
            }

            ui.add_space(10.0);

            ui.label("🎯 Mixed Language Test:");
            let mixed_texts = [
                "English 中文 日本語 한국어",
                "Hello 世界! Welcome 欢迎! ようこそ! 환영합니다!",
                "Numbers 123 English Text 中文 Test 日本語テスト 한국어테스트",
            ];

            for text in mixed_texts {
                ui.label(text);
            }
        });
    }

    #[cfg(feature = "font")]
    fn show_memory_info(&self) {
        println!("\n📊 ===== Memory Info =====");
        println!("📈 Switch Count: {}", self.switch_count);
        println!(
            "⏰ Last Switch: {:.1}s ago",
            self.last_switch_time.elapsed().as_secs_f32()
        );

        if let Some(manager) = &self.font_manager {
            println!("🔤 Current Font: {}", manager.current_font());
            println!("🌐 Current Language: {}", manager.current_language());
        }

        println!("💡 Tips:");
        println!("  - Check Task Manager for memory usage");
        println!("  - Memory should remain stable after multiple switches");
        println!("  - Look for 🔄 and 🧹 in console logs");
        println!("========================\n");
    }

    /// Show placeholder for non-font builds
    #[cfg(not(feature = "font"))]
    fn show_single_font_test(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🔤 Single Font Testing");
        ui.separator();
        ui.colored_label(
            egui::Color32::from_rgb(200, 150, 100),
            "⚠️  Font loading not available in this build",
        );
        ui.label("Run with: --features font");
    }

    #[cfg(not(feature = "font"))]
    fn show_multi_language_support(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🌍 Multi-language Font Support");
        ui.separator();
        ui.colored_label(
            egui::Color32::from_rgb(200, 150, 100),
            "⚠️  Font loading not available in this build",
        );
        ui.label("Run with: --features font");
    }

    #[cfg(not(feature = "font"))]
    fn show_memory_test(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("🧹 Memory Cleanup Test");
        ui.separator();
        ui.colored_label(
            egui::Color32::from_rgb(200, 150, 100),
            "⚠️  Font loading not available in this build",
        );
        ui.label("Run with: --features font");
    }
}

impl eframe::App for ComprehensiveShowcaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Initialize fonts on first frame to ensure clean state
        // 在第一帧初始化字体以确保清洁状态
        #[cfg(feature = "font")]
        static mut INITIALIZED: bool = false;
        #[cfg(feature = "font")]
        if !unsafe { INITIALIZED } {
            self.initialize_fonts(ctx);
            unsafe { INITIALIZED = true };
        }

        // Top panel with tab selector
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎨 egui Theme Showcase - Comprehensive Testing");
                ui.separator();
                ui.label(format!("Theme: {}", self.current_theme.name()));

                #[cfg(feature = "font")]
                if let Some(manager) = &self.font_manager {
                    ui.separator();
                    ui.label(format!("Font: {}", manager.current_font()));
                }
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Tab selector
            self.show_tab_selector(ui);
            ui.add_space(10.0);

            // Tab content
            #[cfg(feature = "font")]
            match self.active_tab {
                Tab::Theme => self.show_theme_showcase(ctx, ui),
                Tab::SingleFont => self.show_single_font_test(ctx, ui),
                Tab::MultiLanguage => self.show_multi_language_support(ctx, ui),
                Tab::MemoryTest => self.show_memory_test(ctx, ui),
            }

            #[cfg(not(feature = "font"))]
            match self.active_tab {
                Tab::Theme => self.show_theme_showcase(ui),
                Tab::SingleFont => self.show_single_font_test(ctx, ui),
                Tab::MultiLanguage => self.show_multi_language_support(ctx, ui),
                Tab::MemoryTest => self.show_memory_test(ctx, ui),
            }
        });
    }
}
