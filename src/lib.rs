//! # egui_kit - 高质量 egui 组件库
//!
//! 提供完整的设计系统、UI组件、扩展方法和实用工具。
//!
//! ## 功能模块
//!
//! - **foundation**: 设计系统 (主题、颜色、设计令牌)
//! - **components**: UI 组件 (按钮、对话框、列表、命令面板等)
//! - **extensions**: egui 扩展方法
//! - **interactions**: 交互系统 (拖放等)
//! - **utils**: 实用工具 (字体加载、日志、Promise、Markdown)
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use egui_kit::foundation::{setup_theme, ThemeName};
//!
//! fn main() -> Result<(), eframe::Error> {
//!     eframe::run_native(
//!         "My App",
//!         eframe::NativeOptions::default(),
//!         Box::new(|cc| {
//!             setup_theme(&cc.egui_ctx, ThemeName::ModernDark);
//!             Ok(Box::new(MyApp::new()))
//!         }),
//!     )
//! }
//! # struct MyApp;
//! # impl MyApp { fn new() -> Self { Self } }
//! # impl eframe::App for MyApp {
//! #     fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
//! # }
//! ```

#![warn(clippy::iter_over_hash_type)]

// 核心模块
pub mod foundation;
pub mod components;
pub mod extensions;
pub mod interactions;
pub mod utils;

#[cfg(feature = "testing")]
pub mod testing;

use egui::NumExt as _;

// ===== Foundation Re-exports =====
pub use foundation::{design_tokens_of, DesignTokens, ThemeName};
pub use foundation::theme::{DARK_THEMES, LIGHT_THEMES, ALL_THEMES};

// ============================================================================
// 主题设置
// ============================================================================

use crate::foundation::theme::{self, style_by_name};

/// Setup theme with ThemeName
///
/// # Example
/// ```rust
/// use egui_kit::{setup_theme, ThemeName};
///
/// fn main() -> Result<(), eframe::Error> {
///     eframe::run_native(
///         "My App",
///         eframe::NativeOptions::default(),
///         Box::new(|cc| {
///             setup_theme(&cc.egui_ctx, ThemeName::ModernDark);
///             Ok(Box::new(MyApp::new()))
///         }),
///     )
/// }
/// ```
pub fn setup_theme(ctx: &egui::Context, name: ThemeName) {
    let style: egui::Style = style_by_name(name);
    ctx.set_style(std::sync::Arc::new(style));
}

/// Convert ThemePreference to ThemeName
impl From<egui::ThemePreference> for ThemeName {
    fn from(preference: egui::ThemePreference) -> Self {
        match preference {
            egui::ThemePreference::Dark => ThemeName::ModernDark,
            egui::ThemePreference::Light => ThemeName::ModernLight,
            egui::ThemePreference::System => ThemeName::ModernDark, // Default to dark for system
        }
    }
}

// ===== Components Re-exports =====
pub use components::{
    // Basic
    Icon,

    // Others
    list_item, SectionCollapsingHeader,
    CommandPalette, CommandPaletteAction, CommandPaletteUrl,
    UICommand, UICommandSender,

    // Dialog
    dialog::Dialog,
};

// ===== Extensions Re-exports =====
pub use extensions::{ContextExt, UiExt, UiLayout};

// ===== Interactions Re-exports =====
pub use interactions::*;

// ---------------------------------------------------------------------------

/// If true, we fill the entire window, except for the close/maximize/minimize buttons in the top-left.
pub fn fullsize_content(os: egui::os::OperatingSystem) -> bool {
    os == egui::os::OperatingSystem::Mac
}

/// If true, we hide the native window decoration
/// (the top bar with app title, close button etc),
/// and instead paint our own close/maximize/minimize buttons.
pub const CUSTOM_WINDOW_DECORATIONS: bool = false;

/// If true, we show the native window decorations/chrome with the
/// close/maximize/minimize buttons and app title.
pub fn native_window_bar(os: egui::os::OperatingSystem) -> bool {
    !fullsize_content(os) && !CUSTOM_WINDOW_DECORATIONS
}

// ----------------------------------------------------------------------------

pub struct TopBarStyle {
    /// Height of the top bar
    pub height: f32,

    /// Extra horizontal space in the top left corner to make room for
    /// close/minimize/maximize buttons (on Mac)
    pub indent: f32,
}

/// The style of a label.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabelStyle {
    /// Regular style for a label.
    #[default]
    Normal,

    /// Label displaying the placeholder text for a yet unnamed item.
    Unnamed,
}

// ----------------------------------------------------------------------------

pub fn design_tokens_of_visuals(visuals: &egui::Visuals) -> &'static DesignTokens {
    if visuals.dark_mode {
        design_tokens_of(egui::Theme::Dark)
    } else {
        design_tokens_of(egui::Theme::Light)
    }
}

pub trait HasDesignTokens {
    fn tokens(&self) -> &'static DesignTokens;
}

impl HasDesignTokens for egui::Context {
    fn tokens(&self) -> &'static DesignTokens {
        design_tokens_of(self.theme())
    }
}

impl HasDesignTokens for egui::Style {
    fn tokens(&self) -> &'static DesignTokens {
        design_tokens_of_visuals(&self.visuals)
    }
}

impl HasDesignTokens for egui::Visuals {
    fn tokens(&self) -> &'static DesignTokens {
        design_tokens_of_visuals(self)
    }
}

/// Apply the design tokens to the given egui context and install image loaders.
pub fn apply_style_and_install_loaders(egui_ctx: &egui::Context) {
    egui_extras::install_image_loaders(egui_ctx);

    egui_ctx.include_bytes(
        "bytes://logo_dark_mode",
        include_bytes!("../assets/logo_dark_mode.png"),
    );
    egui_ctx.include_bytes(
        "bytes://logo_light_mode",
        include_bytes!("../assets/logo_light_mode.png"),
    );

    egui_ctx.options_mut(|o| {
        o.fallback_theme = egui::Theme::Dark;
    });

    set_themes(egui_ctx);

    #[cfg(hot_reload_design_tokens)]
    {
        let egui_ctx = egui_ctx.clone();
        foundation::hot_reload::install_hot_reload(move || {
            log::debug!("Hot-reloading design tokens…");
            foundation::hot_reload::hot_reload_design_tokens();
            set_themes(&egui_ctx);
            egui_ctx.request_repaint();
        });
    }
}

fn set_themes(egui_ctx: &egui::Context) {
    design_tokens_of(egui::Theme::Dark).set_fonts(egui_ctx);

    for theme in [egui::Theme::Dark, egui::Theme::Light] {
        let mut style = std::sync::Arc::unwrap_or_clone(egui_ctx.style_of(theme));
        design_tokens_of(theme).apply(&mut style);
        egui_ctx.set_style_of(theme, style);
    }
}

fn format_with_decimals_in_range(
    value: f64,
    decimal_range: std::ops::RangeInclusive<usize>,
) -> String {
    fn format_with_decimals(value: f64, decimals: usize) -> String {
        format!("{:.decimals$}", value, decimals = decimals)
    }

    let epsilon = 16.0 * f32::EPSILON;

    let min_decimals = *decimal_range.start();
    let max_decimals = *decimal_range.end();
    debug_assert!(min_decimals <= max_decimals);
    debug_assert!(max_decimals < 100);
    let max_decimals = max_decimals.at_most(16);
    let min_decimals = min_decimals.at_most(max_decimals);

    if min_decimals < max_decimals {
        for decimals in min_decimals..max_decimals {
            let text = format_with_decimals(value, decimals);
            if let Some(parsed) = text.parse::<f64>().ok()
                && egui::emath::almost_equal(parsed as f32, value as f32, epsilon)
            {
                return text;
            }
        }
    }

    format_with_decimals(value, max_decimals)
}

/// Is this Ui in a resizable panel?
fn is_in_resizable_panel(ui: &egui::Ui) -> bool {
    let mut is_in_side_panel = false;

    for frame in ui.stack().iter() {
        if let Some(kind) = frame.kind() {
            if kind.is_area() {
                return false;
            }
            if matches!(kind, egui::UiKind::LeftPanel | egui::UiKind::RightPanel) {
                is_in_side_panel = true;
            }
        }
    }

    if is_in_side_panel {
        true
    } else {
        false
    }
}
