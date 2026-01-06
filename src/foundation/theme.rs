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
            // 暗色主题
            ThemePreset::Dark => Self::dark(),
            ThemePreset::DarkBlue => Self::dark_blue(),
            ThemePreset::DarkPurple => Self::dark_purple(),
            ThemePreset::DeepOcean => Self::deep_ocean(),
            ThemePreset::MidnightBlue => Self::midnight_blue(),
            ThemePreset::DarkContrast => Self::dark_contrast(),
            ThemePreset::OneDark => Self::one_dark(),
            ThemePreset::GruvboxDark => Self::gruvbox_dark(),
            ThemePreset::TokyoNight => Self::tokyo_night(),
            ThemePreset::Nord => Self::nord(),
            ThemePreset::Dracula => Self::dracula(),
            ThemePreset::ImAuto => Self::imauto(),
            // 亮色主题
            ThemePreset::Light => Self::light(),
            ThemePreset::LightBlue => Self::light_blue(),
            ThemePreset::SolarizedLight => Self::solarized_light(),
            ThemePreset::LightContrast => Self::light_contrast(),
            ThemePreset::Catppuccin => Self::catppuccin(),
            ThemePreset::GruvboxLight => Self::gruvbox_light(),
            ThemePreset::GitHubLight => Self::github_light(),
        }
    }

    /// Modern dark theme (default)
    pub fn dark() -> Self {
        use super::colors::{ColorPalette, Shade::*};
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
        use super::colors::{ColorPalette, Shade::*};
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
        use super::colors::{ColorPalette, Shade::*};
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
        use super::colors::{ColorPalette, Shade::*};
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

    /// Dark Blue theme - deep blue-black tones
    pub fn dark_blue() -> Self {
        use super::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_dark();

        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: Color32::from_rgb(10, 15, 25),
                surface: Color32::from_rgb(15, 25, 40),
                surface_variant: Color32::from_rgb(25, 35, 50),

                text: palette.gray(S100),
                text_secondary: palette.gray(S400),
                text_disabled: palette.gray(S600),

                primary: palette.blue(S500),
                primary_light: palette.blue(S400),
                primary_dark: palette.blue(S600),
                primary_variant: palette.blue(S800),

                accent: palette.teal(S400),
                success: palette.green(S500),
                warning: palette.yellow(S500),
                error: palette.red(S500),
                info: palette.blue(S400),

                border: palette.gray(S600),
                divider: palette.gray(S700),

                ble_signal_strong: palette.green(S400),
                ble_signal_good: palette.teal(S400),
                ble_signal_weak: palette.yellow(S500),
                ble_signal_very_weak: palette.red(S400),
                ble_connected: palette.green(S500),
                ble_disconnected: palette.gray(S500),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Dark Purple theme - elegant purple-black tones
    pub fn dark_purple() -> Self {
        use super::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_dark();

        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: Color32::from_rgb(15, 10, 20),
                surface: Color32::from_rgb(25, 20, 35),
                surface_variant: Color32::from_rgb(35, 30, 45),

                text: palette.gray(S100),
                text_secondary: palette.gray(S400),
                text_disabled: palette.gray(S600),

                primary: palette.purple(S500),
                primary_light: palette.purple(S400),
                primary_dark: palette.purple(S600),
                primary_variant: palette.purple(S800),

                accent: palette.blue(S400),
                success: palette.green(S500),
                warning: palette.yellow(S500),
                error: palette.red(S500),
                info: palette.teal(S500),

                border: palette.gray(S600),
                divider: palette.gray(S700),

                ble_signal_strong: palette.green(S400),
                ble_signal_good: palette.teal(S400),
                ble_signal_weak: palette.yellow(S500),
                ble_signal_very_weak: palette.red(S400),
                ble_connected: palette.green(S500),
                ble_disconnected: palette.gray(S500),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Deep Ocean theme - deep sea blue-black gradient
    pub fn deep_ocean() -> Self {
        use super::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_dark();

        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: Color32::from_rgb(5, 15, 25),
                surface: Color32::from_rgb(10, 25, 40),
                surface_variant: Color32::from_rgb(15, 35, 55),

                text: palette.gray(S100),
                text_secondary: palette.gray(S400),
                text_disabled: palette.gray(S600),

                primary: palette.blue(S600),
                primary_light: palette.blue(S500),
                primary_dark: palette.blue(S700),
                primary_variant: palette.blue(S800),

                accent: palette.teal(S500),
                success: palette.green(S500),
                warning: palette.yellow(S500),
                error: palette.red(S500),
                info: palette.teal(S400),

                border: palette.gray(S600),
                divider: palette.gray(S700),

                ble_signal_strong: palette.green(S400),
                ble_signal_good: palette.teal(S400),
                ble_signal_weak: palette.yellow(S500),
                ble_signal_very_weak: palette.red(S400),
                ble_connected: palette.green(S500),
                ble_disconnected: palette.gray(S500),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Midnight Blue theme - soft midnight blue-black
    pub fn midnight_blue() -> Self {
        use super::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_dark();

        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: Color32::from_rgb(12, 15, 25),
                surface: Color32::from_rgb(20, 25, 40),
                surface_variant: Color32::from_rgb(30, 35, 50),

                text: palette.gray(S100),
                text_secondary: palette.gray(S400),
                text_disabled: palette.gray(S600),

                primary: palette.blue(S500),
                primary_light: palette.blue(S400),
                primary_dark: palette.blue(S600),
                primary_variant: palette.blue(S800),

                accent: palette.purple(S400),
                success: palette.green(S500),
                warning: palette.yellow(S500),
                error: palette.red(S500),
                info: palette.teal(S500),

                border: palette.gray(S600),
                divider: palette.gray(S700),

                ble_signal_strong: palette.green(S400),
                ble_signal_good: palette.teal(S400),
                ble_signal_weak: palette.yellow(S500),
                ble_signal_very_weak: palette.red(S400),
                ble_connected: palette.green(S500),
                ble_disconnected: palette.gray(S500),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// High contrast dark theme
    pub fn dark_contrast() -> Self {
        use super::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_dark();

        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: Color32::from_rgb(0, 0, 0),
                surface: Color32::from_rgb(20, 20, 20),
                surface_variant: Color32::from_rgb(40, 40, 40),

                text: Color32::from_rgb(255, 255, 255),
                text_secondary: palette.gray(S300),
                text_disabled: palette.gray(S500),

                primary: palette.blue(S400),
                primary_light: palette.blue(S300),
                primary_dark: palette.blue(S500),
                primary_variant: palette.blue(S700),

                accent: palette.purple(S400),
                success: palette.green(S400),
                warning: palette.yellow(S400),
                error: palette.red(S400),
                info: palette.teal(S400),

                border: palette.gray(S500),
                divider: palette.gray(S600),

                ble_signal_strong: palette.green(S400),
                ble_signal_good: palette.teal(S400),
                ble_signal_weak: palette.yellow(S400),
                ble_signal_very_weak: palette.red(S400),
                ble_connected: palette.green(S400),
                ble_disconnected: palette.gray(S400),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// One Dark theme - VS Code classic
    pub fn one_dark() -> Self {
        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: Color32::from_rgb(40, 44, 52),
                surface: Color32::from_rgb(48, 52, 60),
                surface_variant: Color32::from_rgb(56, 60, 68),

                text: Color32::from_rgb(171, 178, 191),
                text_secondary: Color32::from_rgb(130, 135, 145),
                text_disabled: Color32::from_rgb(92, 99, 112),

                primary: Color32::from_rgb(97, 175, 239),
                primary_light: Color32::from_rgb(130, 195, 245),
                primary_dark: Color32::from_rgb(64, 145, 209),
                primary_variant: Color32::from_rgb(40, 100, 150),

                accent: Color32::from_rgb(198, 120, 221),
                success: Color32::from_rgb(152, 195, 121),
                warning: Color32::from_rgb(229, 192, 123),
                error: Color32::from_rgb(224, 108, 117),
                info: Color32::from_rgb(86, 182, 194),

                border: Color32::from_rgb(64, 68, 76),
                divider: Color32::from_rgb(56, 60, 68),

                ble_signal_strong: Color32::from_rgb(152, 195, 121),
                ble_signal_good: Color32::from_rgb(86, 182, 194),
                ble_signal_weak: Color32::from_rgb(229, 192, 123),
                ble_signal_very_weak: Color32::from_rgb(224, 108, 117),
                ble_connected: Color32::from_rgb(152, 195, 121),
                ble_disconnected: Color32::from_rgb(92, 99, 112),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Gruvbox Dark theme - vintage warm dark
    pub fn gruvbox_dark() -> Self {
        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: Color32::from_rgb(40, 40, 40),
                surface: Color32::from_rgb(50, 48, 47),
                surface_variant: Color32::from_rgb(60, 56, 54),

                text: Color32::from_rgb(235, 219, 178),
                text_secondary: Color32::from_rgb(168, 153, 132),
                text_disabled: Color32::from_rgb(124, 111, 100),

                primary: Color32::from_rgb(131, 165, 152),
                primary_light: Color32::from_rgb(169, 182, 101),
                primary_dark: Color32::from_rgb(102, 130, 120),
                primary_variant: Color32::from_rgb(80, 100, 95),

                accent: Color32::from_rgb(211, 134, 155),
                success: Color32::from_rgb(184, 187, 38),
                warning: Color32::from_rgb(250, 189, 47),
                error: Color32::from_rgb(251, 73, 52),
                info: Color32::from_rgb(131, 165, 152),

                border: Color32::from_rgb(80, 73, 69),
                divider: Color32::from_rgb(60, 56, 54),

                ble_signal_strong: Color32::from_rgb(184, 187, 38),
                ble_signal_good: Color32::from_rgb(131, 165, 152),
                ble_signal_weak: Color32::from_rgb(250, 189, 47),
                ble_signal_very_weak: Color32::from_rgb(251, 73, 52),
                ble_connected: Color32::from_rgb(184, 187, 38),
                ble_disconnected: Color32::from_rgb(124, 111, 100),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Tokyo Night theme - popular developer theme
    pub fn tokyo_night() -> Self {
        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: Color32::from_rgb(26, 27, 38),
                surface: Color32::from_rgb(36, 40, 59),
                surface_variant: Color32::from_rgb(46, 50, 69),

                text: Color32::from_rgb(192, 202, 245),
                text_secondary: Color32::from_rgb(130, 137, 173),
                text_disabled: Color32::from_rgb(86, 95, 137),

                primary: Color32::from_rgb(122, 162, 247),
                primary_light: Color32::from_rgb(150, 185, 255),
                primary_dark: Color32::from_rgb(90, 130, 210),
                primary_variant: Color32::from_rgb(60, 90, 150),

                accent: Color32::from_rgb(187, 154, 247),
                success: Color32::from_rgb(158, 206, 106),
                warning: Color32::from_rgb(224, 175, 104),
                error: Color32::from_rgb(247, 118, 142),
                info: Color32::from_rgb(125, 207, 255),

                border: Color32::from_rgb(41, 46, 66),
                divider: Color32::from_rgb(46, 50, 69),

                ble_signal_strong: Color32::from_rgb(158, 206, 106),
                ble_signal_good: Color32::from_rgb(125, 207, 255),
                ble_signal_weak: Color32::from_rgb(224, 175, 104),
                ble_signal_very_weak: Color32::from_rgb(247, 118, 142),
                ble_connected: Color32::from_rgb(158, 206, 106),
                ble_disconnected: Color32::from_rgb(86, 95, 137),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Light Blue theme - fresh blue and white
    pub fn light_blue() -> Self {
        use super::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_light();

        Self {
            preference: egui::ThemePreference::Light,
            colors: ThemeColors {
                background: Color32::from_rgb(240, 248, 255),
                surface: Color32::from_rgb(230, 242, 255),
                surface_variant: Color32::from_rgb(220, 235, 250),

                text: palette.gray(S100),
                text_secondary: palette.gray(S400),
                text_disabled: palette.gray(S600),

                primary: ColorPalette::modern_dark().blue(S600),
                primary_light: ColorPalette::modern_dark().blue(S400),
                primary_dark: ColorPalette::modern_dark().blue(S700),
                primary_variant: ColorPalette::modern_dark().blue(S100),

                accent: ColorPalette::modern_dark().teal(S600),
                success: ColorPalette::modern_dark().green(S600),
                warning: ColorPalette::modern_dark().yellow(S600),
                error: ColorPalette::modern_dark().red(S600),
                info: ColorPalette::modern_dark().blue(S500),

                border: palette.gray(S600),
                divider: palette.gray(S700),

                ble_signal_strong: ColorPalette::modern_dark().green(S500),
                ble_signal_good: ColorPalette::modern_dark().teal(S500),
                ble_signal_weak: ColorPalette::modern_dark().yellow(S600),
                ble_signal_very_weak: ColorPalette::modern_dark().red(S500),
                ble_connected: ColorPalette::modern_dark().green(S600),
                ble_disconnected: ColorPalette::modern_dark().gray(S400),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Solarized Light theme - classic eye-friendly
    pub fn solarized_light() -> Self {
        Self {
            preference: egui::ThemePreference::Light,
            colors: ThemeColors {
                background: Color32::from_rgb(253, 246, 227),
                surface: Color32::from_rgb(238, 232, 213),
                surface_variant: Color32::from_rgb(223, 218, 199),

                text: Color32::from_rgb(101, 123, 131),
                text_secondary: Color32::from_rgb(147, 161, 161),
                text_disabled: Color32::from_rgb(179, 190, 193),

                primary: Color32::from_rgb(38, 139, 210),
                primary_light: Color32::from_rgb(108, 173, 223),
                primary_dark: Color32::from_rgb(7, 102, 175),
                primary_variant: Color32::from_rgb(181, 217, 245),

                accent: Color32::from_rgb(211, 54, 130),
                success: Color32::from_rgb(133, 153, 0),
                warning: Color32::from_rgb(181, 137, 0),
                error: Color32::from_rgb(220, 50, 47),
                info: Color32::from_rgb(42, 161, 152),

                border: Color32::from_rgb(147, 161, 161),
                divider: Color32::from_rgb(223, 218, 199),

                ble_signal_strong: Color32::from_rgb(133, 153, 0),
                ble_signal_good: Color32::from_rgb(42, 161, 152),
                ble_signal_weak: Color32::from_rgb(181, 137, 0),
                ble_signal_very_weak: Color32::from_rgb(220, 50, 47),
                ble_connected: Color32::from_rgb(133, 153, 0),
                ble_disconnected: Color32::from_rgb(147, 161, 161),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// High contrast light theme
    pub fn light_contrast() -> Self {
        use super::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_light();

        Self {
            preference: egui::ThemePreference::Light,
            colors: ThemeColors {
                background: Color32::from_rgb(255, 255, 255),
                surface: Color32::from_rgb(245, 245, 245),
                surface_variant: Color32::from_rgb(230, 230, 230),

                text: Color32::from_rgb(0, 0, 0),
                text_secondary: palette.gray(S300),
                text_disabled: palette.gray(S500),

                primary: Color32::from_rgb(0, 70, 190),
                primary_light: Color32::from_rgb(20, 100, 220),
                primary_dark: Color32::from_rgb(0, 50, 150),
                primary_variant: Color32::from_rgb(200, 220, 255),

                accent: Color32::from_rgb(100, 0, 180),
                success: Color32::from_rgb(0, 130, 0),
                warning: Color32::from_rgb(180, 100, 0),
                error: Color32::from_rgb(200, 0, 0),
                info: Color32::from_rgb(0, 100, 180),

                border: palette.gray(S400),
                divider: palette.gray(S600),

                ble_signal_strong: Color32::from_rgb(0, 150, 0),
                ble_signal_good: Color32::from_rgb(0, 120, 180),
                ble_signal_weak: Color32::from_rgb(200, 120, 0),
                ble_signal_very_weak: Color32::from_rgb(220, 0, 0),
                ble_connected: Color32::from_rgb(0, 150, 0),
                ble_disconnected: palette.gray(S400),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Catppuccin Latte theme - soft latte colors
    pub fn catppuccin() -> Self {
        Self {
            preference: egui::ThemePreference::Light,
            colors: ThemeColors {
                background: Color32::from_rgb(239, 241, 245),
                surface: Color32::from_rgb(230, 233, 239),
                surface_variant: Color32::from_rgb(220, 224, 232),

                text: Color32::from_rgb(76, 79, 105),
                text_secondary: Color32::from_rgb(140, 143, 161),
                text_disabled: Color32::from_rgb(172, 176, 190),

                primary: Color32::from_rgb(30, 102, 245),
                primary_light: Color32::from_rgb(114, 135, 253),
                primary_dark: Color32::from_rgb(4, 65, 179),
                primary_variant: Color32::from_rgb(220, 231, 255),

                accent: Color32::from_rgb(234, 118, 203),
                success: Color32::from_rgb(64, 160, 43),
                warning: Color32::from_rgb(223, 142, 29),
                error: Color32::from_rgb(210, 15, 57),
                info: Color32::from_rgb(32, 159, 181),

                border: Color32::from_rgb(156, 160, 176),
                divider: Color32::from_rgb(220, 224, 232),

                ble_signal_strong: Color32::from_rgb(64, 160, 43),
                ble_signal_good: Color32::from_rgb(32, 159, 181),
                ble_signal_weak: Color32::from_rgb(223, 142, 29),
                ble_signal_very_weak: Color32::from_rgb(210, 15, 57),
                ble_connected: Color32::from_rgb(64, 160, 43),
                ble_disconnected: Color32::from_rgb(140, 143, 161),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Gruvbox Light theme - vintage warm light
    pub fn gruvbox_light() -> Self {
        Self {
            preference: egui::ThemePreference::Light,
            colors: ThemeColors {
                background: Color32::from_rgb(251, 241, 199),
                surface: Color32::from_rgb(242, 229, 188),
                surface_variant: Color32::from_rgb(235, 219, 178),

                text: Color32::from_rgb(60, 56, 54),
                text_secondary: Color32::from_rgb(102, 92, 84),
                text_disabled: Color32::from_rgb(146, 131, 116),

                primary: Color32::from_rgb(69, 133, 136),
                primary_light: Color32::from_rgb(104, 157, 106),
                primary_dark: Color32::from_rgb(40, 100, 103),
                primary_variant: Color32::from_rgb(213, 232, 233),

                accent: Color32::from_rgb(177, 98, 134),
                success: Color32::from_rgb(152, 151, 26),
                warning: Color32::from_rgb(215, 153, 33),
                error: Color32::from_rgb(204, 36, 29),
                info: Color32::from_rgb(69, 133, 136),

                border: Color32::from_rgb(189, 174, 147),
                divider: Color32::from_rgb(235, 219, 178),

                ble_signal_strong: Color32::from_rgb(152, 151, 26),
                ble_signal_good: Color32::from_rgb(69, 133, 136),
                ble_signal_weak: Color32::from_rgb(215, 153, 33),
                ble_signal_very_weak: Color32::from_rgb(204, 36, 29),
                ble_connected: Color32::from_rgb(152, 151, 26),
                ble_disconnected: Color32::from_rgb(146, 131, 116),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// GitHub Light theme - classic GitHub light
    pub fn github_light() -> Self {
        Self {
            preference: egui::ThemePreference::Light,
            colors: ThemeColors {
                background: Color32::from_rgb(255, 255, 255),
                surface: Color32::from_rgb(246, 248, 250),
                surface_variant: Color32::from_rgb(235, 240, 245),

                text: Color32::from_rgb(36, 41, 47),
                text_secondary: Color32::from_rgb(101, 109, 118),
                text_disabled: Color32::from_rgb(149, 157, 165),

                primary: Color32::from_rgb(9, 105, 218),
                primary_light: Color32::from_rgb(54, 140, 244),
                primary_dark: Color32::from_rgb(4, 66, 137),
                primary_variant: Color32::from_rgb(221, 244, 255),

                accent: Color32::from_rgb(130, 80, 223),
                success: Color32::from_rgb(31, 136, 61),
                warning: Color32::from_rgb(191, 135, 0),
                error: Color32::from_rgb(207, 34, 46),
                info: Color32::from_rgb(0, 149, 197),

                border: Color32::from_rgb(208, 215, 222),
                divider: Color32::from_rgb(235, 240, 245),

                ble_signal_strong: Color32::from_rgb(31, 136, 61),
                ble_signal_good: Color32::from_rgb(0, 149, 197),
                ble_signal_weak: Color32::from_rgb(191, 135, 0),
                ble_signal_very_weak: Color32::from_rgb(207, 34, 46),
                ble_connected: Color32::from_rgb(31, 136, 61),
                ble_disconnected: Color32::from_rgb(101, 109, 118),

                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }
}
