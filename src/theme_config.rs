// Theme configuration with semantic color mappings

use super::presets::ThemePreset;
use egui::Color32;

/// Theme configuration with semantic color names
///
/// Instead of using abstract color names like "gray-800",
/// we use semantic names that describe their purpose in the UI
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    /// Theme preference (Dark or Light)
    pub preference: egui::ThemePreference,
    /// Semantic color mappings
    pub colors: ThemeColors,
}

/// Semantic color mappings for UI elements
#[derive(Debug, Clone)]
pub struct ThemeColors {
    // ============================================================================
    // Background Colors
    // ============================================================================
    /// Main application background
    pub background: Color32,

    /// Surface color for panels and cards
    pub surface: Color32,

    /// Variant surface for nested elements
    pub surface_variant: Color32,

    // ============================================================================
    // Text Colors
    // ============================================================================
    /// Primary text color
    pub text: Color32,

    /// Secondary text color (less emphasis)
    pub text_secondary: Color32,

    /// Disabled text color
    pub text_disabled: Color32,

    // ============================================================================
    // Primary Action Colors
    // ============================================================================
    /// Primary brand/action color
    pub primary: Color32,

    /// Lighter variant of primary (hover states)
    pub primary_light: Color32,

    /// Darker variant of primary (pressed states)
    pub primary_dark: Color32,

    /// Very subtle primary for backgrounds
    pub primary_variant: Color32,

    // ============================================================================
    // Accent & Semantic Colors
    // ============================================================================
    /// Secondary accent color
    pub accent: Color32,

    /// Success state (green)
    pub success: Color32,

    /// Warning state (yellow/orange)
    pub warning: Color32,

    /// Error state (red)
    pub error: Color32,

    /// Info state (blue/teal)
    pub info: Color32,

    // ============================================================================
    // Border & Separator Colors
    // ============================================================================
    /// Border color for widgets
    pub border: Color32,

    /// Divider/separator color
    pub divider: Color32,

    // ============================================================================
    // BLE-specific Colors (for RSSI indicators, device states)
    // ============================================================================
    /// Strong BLE signal (RSSI > -50 dBm)
    pub ble_signal_strong: Color32,

    /// Good BLE signal (RSSI -50 to -70 dBm)
    pub ble_signal_good: Color32,

    /// Weak BLE signal (RSSI -70 to -90 dBm)
    pub ble_signal_weak: Color32,

    /// Very weak BLE signal (RSSI < -90 dBm)
    pub ble_signal_very_weak: Color32,

    /// Connected device indicator
    pub ble_connected: Color32,

    /// Disconnected device indicator
    pub ble_disconnected: Color32,

    // ============================================================================
    // ImAuto-specific intermediate colors (用于精确控制层次感)
    // ============================================================================
    /// Floating elements (tooltips, menus) - gray(S250) in im_auto
    pub floating: Option<Color32>,

    /// Faint background - gray(S150) in im_auto
    pub faint_bg: Option<Color32>,

    /// Hovered/open state background - blue(S325) in im_auto
    pub hovered_bg: Option<Color32>,
}

impl ThemeConfig {
    /// Create theme from preset
    pub fn from_preset(preset: ThemePreset) -> Self {
        match preset {
            ThemePreset::Dark => Self::dark(),
            ThemePreset::Light => Self::light(),
            ThemePreset::Nord => Self::nord(),
            ThemePreset::Dracula => Self::dracula(),
            ThemePreset::ImAuto => Self::imauto(),
        }
    }

    /// Modern dark theme (default)
    pub fn dark() -> Self {
        use super::color_palette::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_dark();

        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                // Backgrounds
                background: palette.gray(S900),          // Very dark
                surface: palette.gray(S800),             // Dark panels
                surface_variant: palette.gray(S700),     // Nested elements

                // Text
                text: palette.gray(S100),                // Almost white
                text_secondary: palette.gray(S400),      // Muted
                text_disabled: palette.gray(S600),       // Very muted

                // Primary (Blue)
                primary: palette.blue(S600),             // Vibrant blue
                primary_light: palette.blue(S400),       // Lighter for hover
                primary_dark: palette.blue(S700),        // Darker for press
                primary_variant: palette.blue(S800),     // Subtle background

                // Accent & Semantic
                accent: palette.purple(S500),            // Purple accent
                success: palette.green(S500),            // Green
                warning: palette.yellow(S500),           // Yellow
                error: palette.red(S500),                // Red
                info: palette.teal(S500),                // Teal

                // Borders
                border: palette.gray(S600),              // Visible but subtle
                divider: palette.gray(S700),             // Even more subtle

                // BLE-specific
                ble_signal_strong: palette.green(S400),
                ble_signal_good: palette.teal(S400),
                ble_signal_weak: palette.yellow(S500),
                ble_signal_very_weak: palette.red(S400),
                ble_connected: palette.green(S500),
                ble_disconnected: palette.gray(S500),

                // ImAuto-specific intermediate colors (None for other themes)
                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Modern light theme
    pub fn light() -> Self {
        use super::color_palette::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_light();

        Self {
            preference: egui::ThemePreference::Light,
            colors: ThemeColors {
                // Backgrounds (inverted)
                background: palette.gray(S900),          // Very light
                surface: palette.gray(S800),             // Light panels
                surface_variant: palette.gray(S700),     // Slightly darker

                // Text (inverted)
                text: palette.gray(S100),                // Almost black
                text_secondary: palette.gray(S400),      // Gray
                text_disabled: palette.gray(S600),       // Light gray

                // Primary (Blue - same as dark)
                primary: ColorPalette::modern_dark().blue(S600),
                primary_light: ColorPalette::modern_dark().blue(S400),
                primary_dark: ColorPalette::modern_dark().blue(S700),
                primary_variant: ColorPalette::modern_dark().blue(S100),

                // Accent & Semantic (same as dark)
                accent: ColorPalette::modern_dark().purple(S600),
                success: ColorPalette::modern_dark().green(S600),
                warning: ColorPalette::modern_dark().yellow(S600),
                error: ColorPalette::modern_dark().red(S600),
                info: ColorPalette::modern_dark().teal(S600),

                // Borders (inverted)
                border: palette.gray(S600),
                divider: palette.gray(S700),

                // BLE-specific (same as dark)
                ble_signal_strong: ColorPalette::modern_dark().green(S500),
                ble_signal_good: ColorPalette::modern_dark().teal(S500),
                ble_signal_weak: ColorPalette::modern_dark().yellow(S600),
                ble_signal_very_weak: ColorPalette::modern_dark().red(S500),
                ble_connected: ColorPalette::modern_dark().green(S600),
                ble_disconnected: ColorPalette::modern_dark().gray(S400),

                // ImAuto-specific intermediate colors (None for other themes)
                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Nord theme (popular minimal theme)
    pub fn nord() -> Self {
        use super::color_palette::{ColorPalette, Shade::*};
        let palette = ColorPalette::nord();

        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                // Nord Polar Night
                background: palette.gray(S900),          // #2E3440
                surface: palette.gray(S800),             // #3B4252
                surface_variant: palette.gray(S700),     // #434C5E

                // Nord Snow Storm
                text: palette.gray(S50),                 // #ECEFF4
                text_secondary: palette.gray(S200),      // #D8DEE9
                text_disabled: palette.gray(S300),       // Muted

                // Nord Frost (Blue tones)
                primary: palette.blue(S600),             // #5E81AC
                primary_light: palette.blue(S500),       // #81A1C1
                primary_dark: palette.blue(S700),        // Darker blue
                primary_variant: palette.blue(S800),     // Very dark blue

                // Nord Aurora
                accent: palette.purple(S400),            // #B48EAD
                success: palette.green(S500),            // #A3BE8C
                warning: palette.yellow(S400),           // #EBCB8B
                error: palette.red(S500),                // #BF616A
                info: palette.teal(S400),                // #88C0D0

                // Borders
                border: palette.gray(S600),
                divider: palette.gray(S700),

                // BLE-specific
                ble_signal_strong: palette.green(S400),
                ble_signal_good: palette.teal(S400),
                ble_signal_weak: palette.yellow(S400),
                ble_signal_very_weak: palette.red(S400),
                ble_connected: palette.green(S500),
                ble_disconnected: palette.gray(S500),

                // ImAuto-specific intermediate colors (None for other themes)
                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Dracula theme (vibrant dark theme)
    pub fn dracula() -> Self {
        use super::color_palette::{ColorPalette, Shade::*};
        let palette = ColorPalette::dracula();

        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                // Dracula Background
                background: palette.gray(S900),          // #1E202E (darker)
                surface: palette.gray(S800),             // #282A36 (background)
                surface_variant: palette.gray(S700),     // #44475A (current line)

                // Dracula Foreground
                text: palette.gray(S50),                 // #F8F8F2
                text_secondary: palette.gray(S500),      // #6272A4 (comment)
                text_disabled: palette.gray(S400),       // Muted comment

                // Dracula Purple (primary)
                primary: palette.purple(S400),           // #BD93F9
                primary_light: palette.purple(S300),     // Lighter purple
                primary_dark: palette.purple(S600),      // Darker purple
                primary_variant: palette.purple(S800),   // Very dark purple

                // Dracula Aurora
                accent: palette.red(S400),               // #FF79C6 (pink)
                success: palette.green(S400),            // #50FA7B (green)
                warning: palette.yellow(S400),           // #F1FA8C (yellow)
                error: palette.red(S500),                // #FF5555 (red)
                info: palette.blue(S400),                // #8BE9FD (cyan)

                // Borders
                border: palette.gray(S600),
                divider: palette.gray(S700),

                // BLE-specific
                ble_signal_strong: palette.green(S400),  // Dracula green
                ble_signal_good: palette.blue(S400),     // Dracula cyan
                ble_signal_weak: palette.yellow(S400),   // Dracula yellow
                ble_signal_very_weak: palette.red(S400), // Dracula pink
                ble_connected: palette.green(S400),
                ble_disconnected: palette.gray(S500),

                // ImAuto-specific intermediate colors (None for other themes)
                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// ImAuto Professional theme - extracted from im_auto2 project
    /// Deep blue-gray background with warm yellow accents
    pub fn imauto() -> Self {
        use super::presets::IMAUTO_THEME;
        IMAUTO_THEME.clone()
    }
}
