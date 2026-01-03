//! # egui_kit
//!
//! Elegant theme system for egui applications with zero external dependencies.
//!
//! ## Features
//!
//! - **5 Beautiful Presets**: Modern Dark/Light, Nord, Dracula, and ImAuto Professional themes
//! - **Semantic Colors**: Use meaningful names like `primary`, `success`, `error`
//! - **Runtime Switching**: Change themes without restarting
//! - **Type Safety**: Compile-time checked, zero runtime errors
//! - **Zero Dependencies**: Only requires `egui`, no JSON files or external configs
//! - **BLE-Specific Colors**: Pre-defined colors for signal strength and device states
//! - **Font Loading** (optional): Automatic system font loading with language detection
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use egui_kit::{setup_theme, ThemePreset};
//!
//! fn main() -> Result<(), eframe::Error> {
//!     eframe::run_native(
//!         "My App",
//!         eframe::NativeOptions::default(),
//!         Box::new(|cc| {
//!             // Apply theme on startup
//!             setup_theme(&cc.egui_ctx, ThemePreset::Dark);
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
//!
//! ## Runtime Theme Switching
//!
//! ```rust,no_run
//! use egui_kit::{ThemePreset, ThemeConfig, apply_theme};
//!
//! fn theme_selector(ui: &mut egui::Ui, current: &mut ThemePreset) {
//!     for preset in ThemePreset::all() {
//!         if ui.selectable_label(*current == *preset, preset.name()).clicked() {
//!             *current = *preset;
//!             let theme = ThemeConfig::from_preset(*preset);
//!             apply_theme(ui.ctx(), &theme);
//!         }
//!     }
//! }
//! ```
//!
//! ## Available Themes
//!
//! - **Modern Dark** (default): Clean, professional, inspired by GitHub Dark
//! - **Modern Light**: Excellent readability for daytime use
//! - **Nord**: Minimal arctic-inspired theme, popular among developers
//! - **Dracula**: Vibrant theme with punchy colors
//!
//! ## Custom Colors
//!
//! ```rust
//! use egui_kit::ThemeConfig;
//!
//! let theme = ThemeConfig::from_preset(egui_kit::ThemePreset::Dark);
//! let colors = &theme.colors;
//!
//! // Semantic colors
//! let bg = colors.background;
//! let primary = colors.primary;
//! let success = colors.success;  // Green
//! let error = colors.error;      // Red
//!
//! // BLE-specific colors
//! let strong_signal = colors.ble_signal_strong;
//! let connected = colors.ble_connected;
//! ```

mod color_palette;
mod presets;
mod text;
mod theme_config;

// Dialog module (always available)
pub mod dialog;

// Font loading module (optional feature)
#[cfg(feature = "font")]
pub mod font;

// Promise module for async operations (optional feature)
#[cfg(feature = "promise")]
pub mod promise;

// Logger module (optional feature)
#[cfg(feature = "logger")]
pub mod logger;

// Icons module (optional feature)
// Re-export icon constants and functions for easier access
// 重新导出图标常量和函数，方便使用
#[cfg(feature = "icons")]
pub mod icons {
    /// All icon constants (e.g., SETTINGS_LINE, SEARCH_LINE, etc.)
    pub use egui_remixicon::icons::*;
    /// Add icon fonts to FontDefinitions
    pub use egui_remixicon::add_to_fonts;
    /// Raw font data (for manual font loading)
    pub use egui_remixicon::FONT;
}

use std::sync::Arc;

pub use color_palette::{ColorPalette, Shade};
pub use presets::{DARK_THEME, DRACULA_THEME, IMAUTO_THEME, LIGHT_THEME, NORD_THEME, ThemePreset};
pub use text::{HeadingExt, h1, h2, h3, h4, h5, h6, h7};
pub use theme_config::{ThemeColors, ThemeConfig};

use egui::{Color32, Context, CornerRadius, FontFamily, FontId, Stroke, TextStyle};

/// Quick setup: apply theme from preset
///
/// This is a convenience function that creates a ThemeConfig from the preset
/// and applies it. The theme's preference (Dark/Light) is automatically set.
pub fn setup_theme(ctx: &Context, preset: ThemePreset) {
    let theme = ThemeConfig::from_preset(preset);
    apply_theme(ctx, &theme);
}

/// Enable egui debug styles (hover with all modifiers to see detailed style info)
/// Shows comprehensive debug information including all style properties
/// Only for debug builds
///
/// # Example
/// ```rust,no_run
/// use egui_kit::enable_debug_styles;
///
/// fn setup(ctx: &egui::Context) {
///     #[cfg(debug_assertions)]
///     enable_debug_styles(ctx, true);
/// }
/// ```
pub fn enable_debug_styles(ctx: &Context, enabled: bool) {
    #[cfg(debug_assertions)]
    ctx.all_styles_mut(|style| {
        style.debug.debug_on_hover_with_all_modifiers = enabled;
    });
}

/// Build a Style object from theme configuration
///
/// This function creates a Style without applying it to the context,
/// allowing you to cache the style and apply it later for better performance.
///
/// # Example
///
/// ```
/// // Build style once and cache it
/// let theme_config = ThemeConfig::from_preset(ThemePreset::ImAuto);
/// let cached_style = build_style_from_theme(&theme_config);
///
/// // Apply when needed
/// let original_style = ctx.style().clone();
/// ctx.set_style(cached_style.clone());
/// // ... render UI ...
/// ctx.set_style(original_style);
/// ```
pub fn build_style_from_theme(theme: &ThemeConfig) -> egui::Style {
    // 2. 基于当前主题偏好获取默认样式（确保获取正确的基础主题）
    let mut style = match theme.preference {
        egui::ThemePreference::Dark => {
            let mut s = egui::Style::default();
            s.visuals = egui::Visuals::dark(); // 强制使用 dark visuals
            s
        }
        egui::ThemePreference::Light => {
            let mut s = egui::Style::default();
            s.visuals = egui::Visuals::light(); // 强制使用 light visuals
            s
        }
        _ => {
            let mut s = egui::Style::default();
            s.visuals = egui::Visuals::dark(); // System 默认 dark
            s
        }
    };

    #[cfg(debug_assertions)]
    {
        style.debug.debug_on_hover_with_all_modifiers = true;
    }

    // ============================================================================
    // Typography (与 im_auto2 完全一致)
    // ============================================================================
    style.text_styles = [
        (TextStyle::Small, FontId::new(12.0, FontFamily::Monospace)),
        (TextStyle::Body, FontId::new(14.0, FontFamily::Monospace)),
        (
            TextStyle::Monospace,
            FontId::new(14.0, FontFamily::Monospace),
        ),
        (TextStyle::Button, FontId::new(16.0, FontFamily::Monospace)),
        (TextStyle::Heading, FontId::new(22.0, FontFamily::Monospace)),
        // Heading styles (h1-h7)
        (
            TextStyle::Name("h1".into()),
            FontId::new(32.0, FontFamily::Monospace),
        ),
        (
            TextStyle::Name("h2".into()),
            FontId::new(28.0, FontFamily::Monospace),
        ),
        (
            TextStyle::Name("h3".into()),
            FontId::new(24.0, FontFamily::Monospace),
        ),
        (
            TextStyle::Name("h4".into()),
            FontId::new(20.0, FontFamily::Monospace),
        ),
        (
            TextStyle::Name("h5".into()),
            FontId::new(16.0, FontFamily::Monospace),
        ),
        (
            TextStyle::Name("h6".into()),
            FontId::new(14.0, FontFamily::Monospace),
        ),
        (
            TextStyle::Name("h7".into()),
            FontId::new(12.0, FontFamily::Monospace),
        ),
    ]
    .into();

    // ============================================================================
    // Colors
    // ============================================================================
    let colors = &theme.colors;

    // Background colors (使用 ImAuto 中间层颜色以精确匹配原始主题)
    style.visuals.panel_fill = colors.background; // gray(S100) = panel_bg_color
    style.visuals.window_fill = colors.floating.unwrap_or(colors.surface); // gray(S250) = floating_color (tooltips, menus)
    style.visuals.faint_bg_color = colors.faint_bg.unwrap_or(colors.surface_variant); // gray(S150) = faint_bg_color

    // ============================================================================
    // Widget States (与 im_auto2 完全一致)
    // ============================================================================

    // Noninteractive (Labels, disabled widgets)
    style.visuals.widgets.noninteractive.bg_fill = colors.background;
    style.visuals.widgets.noninteractive.weak_bg_fill = colors.background;
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, colors.border);
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, colors.text);
    style.visuals.widgets.noninteractive.corner_radius = CornerRadius::from(3.0);
    style.visuals.widgets.noninteractive.expansion = 0.0;

    // Inactive (Unhovered button/checkbox)
    style.visuals.widgets.inactive.bg_fill = colors.surface;
    style.visuals.widgets.inactive.weak_bg_fill = colors.surface_variant;
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, colors.border);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.5, colors.text_secondary);
    style.visuals.widgets.inactive.corner_radius = CornerRadius::from(3.0);
    style.visuals.widgets.inactive.expansion = 0.0;

    // Hovered (Mouse over)
    style.visuals.widgets.hovered.bg_fill = colors.primary;
    style.visuals.widgets.hovered.weak_bg_fill = colors.primary_variant;
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, colors.primary_light);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, colors.accent); // 使用 accent (暖黄) 而非 text
    style.visuals.widgets.hovered.corner_radius = CornerRadius::from(3.0);
    style.visuals.widgets.hovered.expansion = 0.0;

    // Active (Button pressed, checkbox checked)
    style.visuals.widgets.active.bg_fill = colors.primary_light;
    style.visuals.widgets.active.weak_bg_fill = colors.primary;
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, colors.primary_light);
    style.visuals.widgets.active.fg_stroke = Stroke::new(2.0, Color32::from_rgb(252, 217, 148)); // active 使用稍暗的暖黄
    style.visuals.widgets.active.corner_radius = CornerRadius::from(3.0);
    style.visuals.widgets.active.expansion = 0.0;

    // Open (Opened menu/dropdown) - 使用 blue(S325) 而非 surface
    style.visuals.widgets.open.bg_fill = colors.hovered_bg.unwrap_or(colors.surface_variant); // blue(S325) = hovered_color
    style.visuals.widgets.open.weak_bg_fill = colors.hovered_bg.unwrap_or(colors.surface); // blue(S325) = hovered_color
    style.visuals.widgets.open.bg_stroke = Stroke::new(1.0, colors.border);
    style.visuals.widgets.open.fg_stroke = Stroke::new(1.0, Color32::from_gray(210)); // open 使用灰色而非纯白
    style.visuals.widgets.open.corner_radius = CornerRadius::from(3.0);
    style.visuals.widgets.open.expansion = 0.0;

    // ============================================================================
    // Selection and Highlights (与 im_auto2 完全一致)
    // ============================================================================
    style.visuals.selection.bg_fill = colors.primary;
    style.visuals.selection.stroke = Stroke::new(2.0, colors.primary_light);

    // Hyperlinks
    style.visuals.hyperlink_color = colors.text_secondary;

    // ============================================================================
    // Special Colors (与 im_auto2 完全一致)
    // ============================================================================
    style.visuals.error_fg_color = Color32::from_rgb(0xAB, 0x01, 0x16);
    style.visuals.warn_fg_color = Color32::from_rgb(0xFF, 0x7A, 0x0C);

    // ============================================================================
    // Window & Panel Settings (与 im_auto2 完全一致)
    // ============================================================================
    style.visuals.window_corner_radius = CornerRadius::from(6.0);
    style.visuals.menu_corner_radius = CornerRadius::from(4.0);
    style.visuals.window_shadow = egui::epaint::Shadow {
        offset: [0, 15],
        blur: 50,
        spread: 0,
        color: Color32::from_black_alpha(128),
    };
    style.visuals.popup_shadow = style.visuals.window_shadow;
    style.visuals.window_stroke = Stroke::NONE;

    // Visual settings
    style.visuals.clip_rect_margin = 0.0;
    style.visuals.striped = false;
    style.visuals.indent_has_left_vline = false;
    style.visuals.image_loading_spinners = false;

    // ============================================================================
    // Spacing (与 im_auto2 完全一致)
    // ============================================================================
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(5.0, 2.0); // im_auto2 的按钮内间距
    style.spacing.indent = 14.0;
    style.spacing.window_margin = egui::Margin {
        left: 8,
        right: 3,
        top: 3,
        bottom: 3,
    };
    style.spacing.menu_margin = egui::Margin::same(6);
    style.spacing.menu_spacing = 1.0;
    style.spacing.text_edit_width = 300.0;
    style.spacing.combo_width = 8.0;
    style.spacing.combo_height = 1000.0;

    // Scrollbar settings (与 im_auto2 完全一致)
    style.spacing.scroll.bar_width = 8.0;
    style.spacing.scroll.bar_inner_margin = 2.0;
    style.spacing.scroll.bar_outer_margin = 2.0;
    style.spacing.scroll.floating = true;
    style.spacing.scroll.floating_width = 3.0;
    style.spacing.scroll.floating_allocated_width = 20.0;

    // Tooltip (与 im_auto2 完全一致)
    style.spacing.tooltip_width = 720.0;
    style.interaction.tooltip_delay = 0.0;

    style
}

/// Apply theme to egui context (legacy API, kept for backward compatibility)
///
/// For better performance, consider using `build_style_from_theme()` to create
/// a cached style that can be reused across frames.
pub fn apply_theme(ctx: &Context, theme: &ThemeConfig) {
    // 1. 先设置主题偏好
    ctx.options_mut(|o| {
        o.theme_preference = theme.preference;
        o.fallback_theme = match theme.preference {
            egui::ThemePreference::Dark => egui::Theme::Dark,
            egui::ThemePreference::Light => egui::Theme::Light,
            egui::ThemePreference::System => egui::Theme::Dark,
        };
    });

    // 2. Build and apply style
    let style = Arc::new(build_style_from_theme(theme));
    ctx.set_style(style);
}
