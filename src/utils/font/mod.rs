//! Font loading system with automatic language detection
//!
//! This module provides automatic font loading based on system language detection.
//! It scans system fonts at startup and selects the best font for the detected language.
//!
//! # Features
//!
//! - Automatic system language detection
//! - Font presets for Chinese, English, Japanese, Korean
//! - Runtime language and font switching
//! - Minimal overhead (only scans font list, doesn't load all fonts)
//!
//! # Example
//!
//! ```rust,no_run
//! use egui_kit::font::FontManager;
//!
//! fn main() -> Result<(), eframe::Error> {
//!     eframe::run_native(
//!         "My App",
//!         eframe::NativeOptions::default(),
//!         Box::new(|cc| {
//!             // Auto-detect language and load font
//!             match FontManager::new(&cc.egui_ctx) {
//!                 Ok(font_manager) => {
//!                     println!("Loaded font: {}", font_manager.current_font());
//!                 }
//!                 Err(e) => {
//!                     eprintln!("Font loading failed: {}", e);
//!                 }
//!             }
//!             Ok(Box::<MyApp>::default())
//!         }),
//!     )
//! }
//! # struct MyApp;
//! # impl eframe::App for MyApp {
//! #     fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
//! # }
//! ```

use std::fmt;

// ============================================================================
// Chinese Font Embedded Data (WASM + chinese feature)
// ============================================================================

#[cfg(all(target_arch = "wasm32", feature = "chinese"))]
const CHINESE_FONT_DATA: &[u8] = include_bytes!("../../../assets/DroidSansFallbackFull.ttf");

// ============================================================================
// Public Types
// ============================================================================

/// Supported languages for font selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// Chinese (Simplified and Traditional)
    Chinese,
    /// English
    English,
    /// Japanese
    Japanese,
    /// Korean
    Korean,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Chinese => write!(f, "Chinese"),
            Language::English => write!(f, "English"),
            Language::Japanese => write!(f, "Japanese"),
            Language::Korean => write!(f, "Korean"),
        }
    }
}

/// Loaded font data
#[derive(Clone)]
pub struct LoadedFont {
    /// Font family name
    pub family_name: String,
    /// Font data (TTF/OTF bytes)
    pub data: Vec<u8>,
}

/// Font loading errors
#[derive(Debug)]
pub enum FontError {
    /// Failed to scan system fonts
    ScanFailed(String),
    /// Font family not found
    FontNotFound(String),
    /// Font family has no fonts
    NoFontInFamily(String),
    /// Failed to load font file
    LoadFailed(String),
    /// No suitable font found for language
    NoSuitableFont(Language),
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontError::ScanFailed(msg) => write!(f, "Failed to scan system fonts: {}", msg),
            FontError::FontNotFound(name) => write!(f, "Font not found: {}", name),
            FontError::NoFontInFamily(name) => write!(f, "No fonts in family: {}", name),
            FontError::LoadFailed(msg) => write!(f, "Failed to load font: {}", msg),
            FontError::NoSuitableFont(lang) => {
                write!(f, "No suitable font found for language: {}", lang)
            }
        }
    }
}

impl std::error::Error for FontError {}

/// Font manager - manages font loading and switching
///
/// This is the main entry point for the font loading system.
/// Supports both single language mode and multi-language fallback mode.
/// Create one instance at application startup and keep it alive.
pub struct FontManager {
    /// Font loader (only available with system-font feature)
    #[cfg(feature = "system-font")]
    loader: FontLoader,
    /// Available system fonts (only available with system-font feature)
    #[cfg(feature = "system-font")]
    available_fonts: Vec<String>,
    /// Available system fonts (empty for chinese-only mode)
    #[cfg(not(feature = "system-font"))]
    available_fonts: Vec<String>,
    // Single language mode (current)
    current_language: Language,
    current_font: String,
    previous_font: Option<String>,
    // Multi-language mode (new)
    selected_languages: Vec<Language>,
    multi_language_fonts: Vec<String>,
    is_multi_language_mode: bool,
}

impl FontManager {
    /// Initialize font manager with auto-detected language
    ///
    /// This will:
    /// 1. For Native + system-font: Detect system language and scan system fonts
    /// 2. For WASM + chinese: Load embedded Chinese TTF font automatically
    /// 3. Load and apply font to egui context
    ///
    /// # Errors
    ///
    /// Returns error if font scanning or loading fails
    pub fn new(ctx: &egui::Context) -> Result<Self, FontError> {
        // With system-font feature (full functionality for native targets)
        #[cfg(feature = "system-font")]
        {
            let language = detect_system_language();
            let mut loader = FontLoader::new()?;
            let available_fonts = loader.scan_system_fonts()?;
            log::info!("Found {} system fonts", available_fonts.len());

            let font = loader.load_best_font_for_language(language)?;
            log::info!("Loaded font: {} for {}", font.family_name, language);

            apply_font(ctx, &font, None, egui::FontFamily::Proportional)?;
            apply_font(ctx, &font, None, egui::FontFamily::Monospace)?;

            return Ok(Self {
                loader,
                available_fonts,
                current_language: language,
                current_font: font.family_name.clone(),
                previous_font: None,
                selected_languages: vec![language],
                multi_language_fonts: vec![font.family_name.clone()],
                is_multi_language_mode: false,
            });
        }

        // WASM + chinese feature: Load embedded Chinese TTF automatically
        #[cfg(all(target_arch = "wasm32", feature = "chinese"))]
        {
            log::info!("WASM + chinese: Loading embedded Chinese font");

            // Create font data from embedded TTF
            let font_data = CHINESE_FONT_DATA.to_vec();
            let font_name = "DroidSansFallback".to_string();

            // Setup font definitions with embedded font
            let mut fonts = egui::FontDefinitions::default();

            // Add icon fonts first (if icons feature is enabled)
            #[cfg(feature = "icons")]
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

            // Insert Chinese font
            fonts.font_data.insert(
                font_name.clone(),
                std::sync::Arc::new(egui::FontData::from_owned(font_data)),
            );

            // Add to both font families for proper CJK rendering
            for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                if let Some(font_list) = fonts.families.get_mut(&family) {
                    font_list.push(font_name.clone());
                }
            }

            // Add Phosphor icon font to Monospace family
            #[cfg(feature = "icons")]
            {
                if let Some(font_list) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                    font_list.insert(1, "phosphor".to_string());
                }
            }

            ctx.set_fonts(fonts);
            ctx.request_repaint();

            log::info!("WASM + chinese: Embedded Chinese font loaded successfully");

            return Ok(Self {
                #[cfg(feature = "system-font")]
                loader: FontLoader::new()?,
                #[cfg(feature = "system-font")]
                available_fonts: Vec::new(),
                #[cfg(not(feature = "system-font"))]
                available_fonts: Vec::new(),
                current_language: Language::Chinese,
                current_font: font_name.clone(),
                previous_font: None,
                selected_languages: vec![Language::Chinese],
                multi_language_fonts: vec![font_name],
                is_multi_language_mode: false,
            });
        }

        // No suitable configuration - return Ok with empty fonts (use egui defaults)
        #[cfg(not(any(feature = "system-font", all(target_arch = "wasm32", feature = "chinese"))))]
        {
            log::info!("FontManager: No font features enabled, using egui default fonts");
            return Ok(Self {
                #[cfg(feature = "system-font")]
                loader: FontLoader::new()?,
                #[cfg(feature = "system-font")]
                available_fonts: Vec::new(),
                #[cfg(not(feature = "system-font"))]
                available_fonts: Vec::new(),
                current_language: Language::English,
                current_font: "default".to_string(),
                previous_font: None,
                selected_languages: vec![],
                multi_language_fonts: vec![],
                is_multi_language_mode: false,
            });
        }
    }

    /// Get all available system fonts
    pub fn available_fonts(&self) -> &[String] {
        &self.available_fonts
    }

    /// Get current language
    pub fn current_language(&self) -> Language {
        self.current_language
    }

    /// Get current font name
    pub fn current_font(&self) -> &str {
        &self.current_font
    }

    /// Switch to different language (auto-selects best font)
    ///
    /// # Errors
    ///
    /// Returns error if no suitable font found for language
    #[cfg(feature = "system-font")]
    pub fn switch_language(
        &mut self,
        ctx: &egui::Context,
        language: Language,
    ) -> Result<(), FontError> {
        if self.current_language == language {
            return Ok(()); // Already using this language
        }

        let font = self.loader.load_best_font_for_language(language)?;

        // Apply font with memory cleanup - pass previous font for cleanup tracking
        // 应用字体并清理内存 - 传递之前的字体用于清理跟踪
        apply_font(
            ctx,
            &font,
            self.previous_font.as_deref(),
            egui::FontFamily::Proportional,
        )?;

        // Update font tracking
        // 更新字体跟踪
        self.previous_font = Some(self.current_font.clone());
        self.current_language = language;
        self.current_font = font.family_name;
        log::info!(
            "Switched to language: {}, font: {}",
            language,
            self.current_font
        );

        Ok(())
    }

    /// Switch to specific font (manual selection)
    ///
    /// # Errors
    ///
    /// Returns error if font not found or loading fails
    #[cfg(feature = "system-font")]
    pub fn switch_font(&mut self, ctx: &egui::Context, family_name: &str) -> Result<(), FontError> {
        let font = self.loader.load_font(family_name)?;

        // Apply font with memory cleanup - pass previous font for cleanup tracking
        // 应用字体并清理内存 - 传递之前的字体用于清理跟踪
        apply_font(
            ctx,
            &font,
            self.previous_font.as_deref(),
            egui::FontFamily::Proportional,
        )?;

        // Update font tracking
        // 更新字体跟踪
        self.previous_font = Some(self.current_font.clone());
        self.current_font = family_name.to_string();
        log::info!("Switched to font: {}", self.current_font);

        Ok(())
    }

    // ============================================================================
    // Multi-language Support (New)
    // ============================================================================

    /// Enable/disable multi-language mode
    /// 启用/禁用多语言模式
    pub fn set_multi_language_mode(&mut self, enabled: bool) {
        if self.is_multi_language_mode != enabled {
            self.is_multi_language_mode = enabled;
            log::info!(
                "Multi-language mode: {}",
                if enabled { "ON" } else { "OFF" }
            );
        }
    }

    /// Check if multi-language mode is enabled
    /// 检查多语言模式是否启用
    pub fn is_multi_language_mode(&self) -> bool {
        self.is_multi_language_mode
    }

    /// Get selected languages
    /// 获取选中的语言列表
    pub fn selected_languages(&self) -> &[Language] {
        &self.selected_languages
    }

    /// Add language to multi-language support
    /// 添加语言到多语言支持
    pub fn add_language(&mut self, language: Language) -> bool {
        if !self.selected_languages.contains(&language) {
            self.selected_languages.push(language);
            log::info!("Added language to multi-language support: {}", language);
            true
        } else {
            false
        }
    }

    /// Remove language from multi-language support
    /// 从多语言支持中移除语言
    pub fn remove_language(&mut self, language: Language) -> bool {
        if let Some(index) = self
            .selected_languages
            .iter()
            .position(|&lang| lang == language)
        {
            self.selected_languages.remove(index);
            log::info!("Removed language from multi-language support: {}", language);
            true
        } else {
            false
        }
    }

    /// Apply multi-language fonts to context
    /// 应用多语言字体到上下文
    #[cfg(feature = "system-font")]
    pub fn apply_multi_language_fonts(&mut self, ctx: &egui::Context) -> Result<(), FontError> {
        if !self.is_multi_language_mode {
            return Err(FontError::NoSuitableFont(Language::English)); // Use English as error placeholder
        }

        if self.selected_languages.is_empty() {
            return Err(FontError::NoSuitableFont(Language::English));
        }

        // Load fonts for all selected languages
        let mut loaded_fonts = Vec::new();
        for &language in &self.selected_languages {
            match self.loader.load_best_font_for_language(language) {
                Ok(font) => {
                    loaded_fonts.push(font);
                    log::debug!(
                        "Loaded font for {}: {}",
                        language,
                        loaded_fonts.last().unwrap().family_name
                    );
                }
                Err(e) => {
                    log::warn!("Failed to load font for {}: {}", language, e);
                }
            }
        }

        if loaded_fonts.is_empty() {
            return Err(FontError::NoSuitableFont(Language::English));
        }

        // Apply multi-language fonts with proper fallback chain
        apply_multi_language_fonts(ctx, &loaded_fonts)?;

        // Update font tracking
        self.multi_language_fonts = loaded_fonts.iter().map(|f| f.family_name.clone()).collect();
        log::info!(
            "Applied {} fonts for multi-language support",
            self.multi_language_fonts.len()
        );

        Ok(())
    }

    /// Get multi-language font information
    /// 获取多语言字体信息
    pub fn multi_language_info(&self) -> (&[Language], &[String]) {
        (&self.selected_languages, &self.multi_language_fonts)
    }

    /// Synchronize selected languages and apply fonts
    /// This method completely resets the font system and rebuilds from scratch
    /// 同步选择的语言并应用字体
    /// 此方法完全重置字体系统并从头开始重建
    #[cfg(feature = "system-font")]
    pub fn sync_selected_languages(
        &mut self,
        ctx: &egui::Context,
        selected_languages: &[Language],
    ) -> Result<(), FontError> {
        log::debug!("Syncing languages: {:?}", selected_languages);

        // Reset selected languages tracking
        // 重置选择的语言跟踪
        self.selected_languages.clear();
        self.multi_language_fonts.clear();

        if selected_languages.is_empty() {
            log::debug!("No languages selected, using egui default fonts");
            // Apply egui default fonts when no language is selected
            // 当没有选择语言时，应用egui默认字体
            ctx.set_fonts(egui::FontDefinitions::default());
            return Ok(());
        }

        // Update selected languages tracking
        // 更新选择的语言跟踪
        for &language in selected_languages {
            self.selected_languages.push(language);
        }

        // Apply fonts based on current mode
        // 根据当前模式应用字体
        if self.is_multi_language_mode {
            // Multi-language mode: load all selected fonts
            // 多语言模式：加载所有选择的字体
            let mut loaded_fonts = Vec::new();
            for &language in &self.selected_languages {
                match self.loader.load_best_font_for_language(language) {
                    Ok(font) => {
                        let font_name = font.family_name.clone();
                        loaded_fonts.push(font);
                        log::debug!("Loaded font for {}: {}", language, font_name);
                    }
                    Err(e) => {
                        log::warn!("Failed to load font for {}: {}", language, e);
                    }
                }
            }

            if !loaded_fonts.is_empty() {
                // Apply multi-language fonts from scratch
                // 从头开始应用多语言字体
                apply_multi_language_fonts(ctx, &loaded_fonts)?;
                self.multi_language_fonts =
                    loaded_fonts.iter().map(|f| f.family_name.clone()).collect();
            } else {
                log::warn!("No fonts could be loaded, using egui defaults");
                ctx.set_fonts(egui::FontDefinitions::default());
            }
        } else {
            // Single language mode: use first selected language
            // 单语言模式：使用第一个选择的语言
            if let Some(&language) = self.selected_languages.first() {
                match self.loader.load_best_font_for_language(language) {
                    Ok(font) => {
                        let font_name = font.family_name.clone();
                        apply_font(
                            ctx,
                            &font,
                            self.previous_font.as_deref(),
                            egui::FontFamily::Proportional,
                        )?;
                        self.current_language = language;
                        self.current_font = font_name;
                        log::debug!(
                            "Applied single language font: {} for {}",
                            self.current_font,
                            language
                        );
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to load font for {}: {}, using egui defaults",
                            language,
                            e
                        );
                        ctx.set_fonts(egui::FontDefinitions::default());
                    }
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Font Loader (Internal)
// ============================================================================

struct FontLoader {
    #[cfg(feature = "system-font")]
    system_source: font_kit::source::SystemSource,
}

impl FontLoader {
    fn new() -> Result<Self, FontError> {
        Ok(Self {
            #[cfg(feature = "system-font")]
            system_source: font_kit::source::SystemSource::new(),
        })
    }

    /// Scan system fonts and return family names (doesn't load data)
    #[cfg(feature = "system-font")]
    fn scan_system_fonts(&mut self) -> Result<Vec<String>, FontError> {
        let families = self
            .system_source
            .all_families()
            .map_err(|e| FontError::ScanFailed(e.to_string()))?;

        let mut font_list: Vec<String> = families.into_iter().collect();
        font_list.sort(); // Sort alphabetically for UI display
        Ok(font_list)
    }

    /// Load best font for specified language
    #[cfg(feature = "system-font")]
    fn load_best_font_for_language(&self, language: Language) -> Result<LoadedFont, FontError> {
        let presets = FontPresets::for_language(language);

        // Try each preset in priority order
        for family_name in presets {
            match load_font_by_name(family_name) {
                Ok(font) => return Ok(font),
                Err(e) => {
                    log::debug!("Failed to load preset font '{}': {}", family_name, e);
                }
            }
        }

        Err(FontError::NoSuitableFont(language))
    }

    /// Load specific font by family name
    #[cfg(feature = "system-font")]
    fn load_font(&self, family_name: &str) -> Result<LoadedFont, FontError> {
        let family = self
            .system_source
            .select_family_by_name(family_name)
            .map_err(|_| FontError::FontNotFound(family_name.to_string()))?;

        let handle = family
            .fonts()
            .first()
            .ok_or_else(|| FontError::NoFontInFamily(family_name.to_string()))?;

        let data = match handle {
            font_kit::handle::Handle::Path { path, .. } => {
                std::fs::read(path).map_err(|e| FontError::LoadFailed(e.to_string()))?
            }
            font_kit::handle::Handle::Memory { bytes, .. } => bytes.to_vec(),
        };

        Ok(LoadedFont {
            family_name: family_name.to_string(),
            data,
        })
    }
}

// ============================================================================
// Font Presets
// ============================================================================

struct FontPresets;

impl FontPresets {
    /// Chinese font presets (priority order)
    const CHINESE: &'static [&'static str] = &[
        "SimHei",          // 黑体
        "Microsoft YaHei", // 微软雅黑 (Windows default)
        "Microsoft YaHei UI",
        "SimSun",  // 宋体
        "NSimSun", // 新宋体
        "KaiTi",   // 楷体
    ];

    /// English font presets (priority order)
    const ENGLISH: &'static [&'static str] = &[
        "Segoe UI", // Windows 10/11 default
        "Arial", "Calibri", "Tahoma", "Verdana", "Consolas",
    ];

    /// Japanese font presets (priority order)
    const JAPANESE: &'static [&'static str] = &[
        "Yu Gothic UI", // Windows 10/11 default
        "Yu Gothic",
        "Meiryo UI",
        "Meiryo",
        "MS Gothic",
        "MS UI Gothic",
    ];

    /// Korean font presets (priority order)
    const KOREAN: &'static [&'static str] = &[
        "Malgun Gothic", // Windows default
        "Gulim",
        "Dotum",
        "Batang",
    ];

    /// Get preset list for language
    fn for_language(language: Language) -> &'static [&'static str] {
        match language {
            Language::Chinese => Self::CHINESE,
            Language::English => Self::ENGLISH,
            Language::Japanese => Self::JAPANESE,
            Language::Korean => Self::KOREAN,
        }
    }
}

// ============================================================================
// Load font by name using font-kit (internal helper)
// ============================================================================

/// Load font by name using font-kit (internal helper)
#[cfg(feature = "system-font")]
fn load_font_by_name(family_name: &str) -> Result<LoadedFont, FontError> {
    let family = font_kit::source::SystemSource::new()
        .select_family_by_name(family_name)
        .map_err(|_| FontError::FontNotFound(family_name.to_string()))?;

    let handle = family
        .fonts()
        .first()
        .ok_or_else(|| FontError::NoFontInFamily(family_name.to_string()))?;

    let data = match handle {
        font_kit::handle::Handle::Path { path, .. } => {
            std::fs::read(path).map_err(|e| FontError::LoadFailed(e.to_string()))?
        }
        font_kit::handle::Handle::Memory { bytes, .. } => bytes.to_vec(),
    };

    Ok(LoadedFont {
        family_name: family_name.to_string(),
        data,
    })
}

// ============================================================================
// System Language Detection
// ============================================================================

/// Detect system language using sys-locale
#[cfg(feature = "system-font")]
pub fn detect_system_language() -> Language {
    let locale = get_locale().unwrap_or_else(|| "en-US".to_string());
    log::debug!("System locale: {}", locale);

    // Parse locale string (e.g., "zh-CN", "en-US", "ja-JP")
    if locale.starts_with("zh") {
        Language::Chinese
    } else if locale.starts_with("ja") {
        Language::Japanese
    } else if locale.starts_with("ko") {
        Language::Korean
    } else {
        Language::English // Default fallback
    }
}

// ============================================================================
// egui Integration
// ============================================================================

/// Apply font to egui context with memory cleanup
///
/// This function implements font memory cleanup to prevent font data accumulation.
/// Based on the working font loading code from im_auto2 project.
/// 这实现了字体内存清理，防止字体数据累积。基于 im_auto2 项目的可运行代码。
fn apply_font(
    ctx: &egui::Context,
    font: &LoadedFont,
    previous_font: Option<&str>,
    _family: egui::FontFamily,
) -> Result<(), FontError> {
    // Always start from default font definitions
    // This ensures egui's built-in fonts are always available and clears old font data
    // 始终从默认字体定义开始，确保 egui 内置字体始终可用并清理旧字体数据
    let mut fonts = egui::FontDefinitions::default();

    // Add icon fonts first (if icons feature is enabled)
    // 首先添加图标字体 (如果启用了 icons 特性)
    #[cfg(feature = "icons")]
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

    // MEMORY CLEANUP: Log font switching for debugging
    // 内存清理：记录字体切换用于调试
    if let Some(prev_font) = previous_font {
        if prev_font != font.family_name {
            log::debug!("Font switch: {} -> {}", prev_font, font.family_name);
        } else {
            log::debug!("Font reloaded: {}", font.family_name);
        }
    } else {
        log::debug!("Initial font: {}", font.family_name);
    }

    // Insert new font data only
    // 只插入新字体数据
    fonts.font_data.insert(
        font.family_name.clone(),
        std::sync::Arc::new(egui::FontData::from_owned(font.data.clone())),
    );

    // CRITICAL: Add Phosphor icon font to Monospace family
    // 注意：Proportional 家族由 egui_phosphor::add_to_fonts() 自动处理
    // 关键：将 Phosphor 图标字体添加到 Monospace 家族用于图标渲染
    #[cfg(feature = "icons")]
    {
        if let Some(font_list) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
            // Insert at position 1, after default font, for proper icon rendering
            // 插入到位置 1（在默认字体之后），确保图标正确渲染
            font_list.insert(1, "phosphor".to_string());
        }
    }

    // CRITICAL INSIGHT: Based on working reference code
    // 关键洞察：基于可运行的参考代码
    // For CJK fonts to work correctly, we need to:
    // 1. Add to BOTH Proportional and Monospace families
    // 2. Use push() instead of insert(0) to maintain proper fallback order
    // 3. Let egui's built-in fonts try first, then fallback to our CJK font
    // 为使 CJK 字体正确工作，我们需要：
    // 1. 同时添加到 Proportional 和 Monospace 字体家族
    // 2. 使用 push() 而不是 insert(0) 来保持正确的回退顺序
    // 3. 让 egui 内置字体先尝试，然后回退到我们的 CJK 字体

    let font_families = [egui::FontFamily::Proportional, egui::FontFamily::Monospace];

    for target_family in font_families {
        if let Some(font_list) = fonts.families.get_mut(&target_family) {
            // IMPORTANT: Use push() to add to END, not beginning
            // 重要：使用 push() 添加到末尾，不是开头
            // This creates proper fallback: egui defaults -> our CJK font
            // 这创建了正确的回退：egui 默认字体 -> 我们的 CJK 字体
            font_list.push(font.family_name.clone());
        }
    }

    // Apply to context (this replaces all previous font definitions)
    // 应用到上下文（这会替换所有之前的字体定义）
    ctx.set_fonts(fonts);

    // MEMORY CLEANUP: Force font texture atlas rebuild to release old font memory
    // 内存清理：强制重建字体纹理图集以释放旧字体内存
    ctx.request_repaint();

    // Log memory optimization
    // 记录内存优化
    log::debug!("Font memory cleanup completed");

    Ok(())
}

/// Apply multiple fonts to egui context with proper fallback chain
/// 应用多个字体到 egui 上下文，建立正确的回退链
///
/// This function creates a font fallback chain that allows egui to try each font
/// in order when rendering characters, ensuring proper display of mixed-language text.
/// 这个函数创建字体回退链，让 egui 在渲染字符时按顺序尝试每个字体，
/// 确保混合语言文本的正确显示。
fn apply_multi_language_fonts(ctx: &egui::Context, fonts: &[LoadedFont]) -> Result<(), FontError> {
    if fonts.is_empty() {
        return Err(FontError::NoSuitableFont(Language::English));
    }

    // Start with empty font definitions for strict multi-language mode
    // 为严格多语言模式从空字体定义开始
    let mut font_definitions = egui::FontDefinitions::default();

    // Clear default fonts to ensure only user-selected fonts are used
    // 清除默认字体以确保只使用用户选择的字体
    for font_list in font_definitions.families.values_mut() {
        font_list.clear();
    }

    // Add icon fonts AFTER clearing (if icons feature is enabled)
    // 在清除后添加图标字体 (如果启用了 icons 特性)
    #[cfg(feature = "icons")]
    egui_phosphor::add_to_fonts(&mut font_definitions, egui_phosphor::Variant::Regular);

    // CRITICAL: Add Phosphor icon font to Monospace family
    // 注意：Proportional 家族由 egui_phosphor::add_to_fonts() 自动处理
    #[cfg(feature = "icons")]
    {
        if let Some(font_list) = font_definitions.families.get_mut(&egui::FontFamily::Monospace) {
            // Insert at position 1, after default font, for proper icon rendering
            font_list.insert(1, "phosphor".to_string());
        }
    }

    // Log multi-language font setup
    // 记录多语言字体设置
    log::debug!("Setting up strict multi-language fonts:");
    for (i, font) in fonts.iter().enumerate() {
        log::debug!(
            "  {}. {} ({})",
            i + 1,
            font.family_name,
            if i == 0 { "primary" } else { "fallback" }
        );

        // Insert font data
        // 插入字体数据
        font_definitions.font_data.insert(
            font.family_name.clone(),
            std::sync::Arc::new(egui::FontData::from_owned(font.data.clone())),
        );
    }

    // Create strict fallback chain for both Proportional and Monospace families
    // 为 Proportional 和 Monospace 字体家族创建严格的回退链
    let font_families = [egui::FontFamily::Proportional, egui::FontFamily::Monospace];

    for target_family in font_families {
        if let Some(font_list) = font_definitions.families.get_mut(&target_family) {
            // ONLY use user-selected fonts, no egui defaults
            // 只使用用户选择的字体，不包含 egui 默认字体
            for font in fonts {
                font_list.push(font.family_name.clone());
            }
        }
    }

    // Apply to context
    // 应用到上下文
    ctx.set_fonts(font_definitions);

    // Force font texture atlas rebuild
    // 强制重建字体纹理图集
    ctx.request_repaint();

    log::debug!(
        "Strict multi-language font setup completed with {} fonts",
        fonts.len()
    );

    Ok(())
}
