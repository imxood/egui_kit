//! 主题样式模块
//!
//! 基于 egui::Style 的完整主题定义, 支持 15 种预设主题.
//! 所有参数都提供中文释义, 部分参数可使用 egui 默认值.
//!
//! # 模块结构
//!
//! - [`style::dark`] - 10 套暗色主题
//! - [`style::light`] - 5 套亮色主题
//!

pub mod style;
pub use style::{ThemeStyle, DARK_THEMES, LIGHT_THEMES, ALL_THEMES};

// =============================================================================
// ThemeConfig (遗留 API: 基于语义化颜色的轻量级主题)
// =============================================================================

use egui::Color32;

/// Theme configuration with semantic color names
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
    /// Main application background
    pub background: Color32,
    /// Surface color for panels and cards
    pub surface: Color32,
    /// Variant surface for nested elements
    pub surface_variant: Color32,
    /// Primary text color
    pub text: Color32,
    /// Secondary text color
    pub text_secondary: Color32,
    /// Disabled text color
    pub text_disabled: Color32,
    /// Primary brand/action color
    pub primary: Color32,
    /// Lighter variant of primary (hover states)
    pub primary_light: Color32,
    /// Darker variant of primary (pressed states)
    pub primary_dark: Color32,
    /// Very subtle primary for backgrounds
    pub primary_variant: Color32,
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
    /// Border color for widgets
    pub border: Color32,
    /// Divider/separator color
    pub divider: Color32,
    /// BLE signal strong
    pub ble_signal_strong: Color32,
    /// BLE signal good
    pub ble_signal_good: Color32,
    /// BLE signal weak
    pub ble_signal_weak: Color32,
    /// BLE signal very weak
    pub ble_signal_very_weak: Color32,
    /// BLE connected
    pub ble_connected: Color32,
    /// BLE disconnected
    pub ble_disconnected: Color32,
    /// Floating elements
    pub floating: Option<Color32>,
    /// Faint background
    pub faint_bg: Option<Color32>,
    /// Hovered/open state background
    pub hovered_bg: Option<Color32>,
}

impl ThemeConfig {
    /// Create theme from preset
    pub fn from_preset(preset: crate::foundation::presets::ThemePreset) -> Self {
        match preset {
            crate::foundation::presets::ThemePreset::Dark => Self::dark(),
            crate::foundation::presets::ThemePreset::DarkBlue => Self::dark_blue(),
            crate::foundation::presets::ThemePreset::DarkPurple => Self::dark_purple(),
            crate::foundation::presets::ThemePreset::DeepOcean => Self::deep_ocean(),
            crate::foundation::presets::ThemePreset::MidnightBlue => Self::midnight_blue(),
            crate::foundation::presets::ThemePreset::DarkContrast => Self::dark_contrast(),
            crate::foundation::presets::ThemePreset::OneDark => Self::one_dark(),
            crate::foundation::presets::ThemePreset::GruvboxDark => Self::gruvbox_dark(),
            crate::foundation::presets::ThemePreset::TokyoNight => Self::tokyo_night(),
            crate::foundation::presets::ThemePreset::Nord => Self::nord(),
            crate::foundation::presets::ThemePreset::Dracula => Self::dracula(),
            crate::foundation::presets::ThemePreset::ImAuto => Self::imauto(),
            crate::foundation::presets::ThemePreset::Light => Self::light(),
            crate::foundation::presets::ThemePreset::LightBlue => Self::light_blue(),
            crate::foundation::presets::ThemePreset::SolarizedLight => Self::solarized_light(),
            crate::foundation::presets::ThemePreset::LightContrast => Self::light_contrast(),
            crate::foundation::presets::ThemePreset::Catppuccin => Self::catppuccin(),
            crate::foundation::presets::ThemePreset::GruvboxLight => Self::gruvbox_light(),
            crate::foundation::presets::ThemePreset::GitHubLight => Self::github_light(),
        }
    }

    /// Modern dark theme (default)
    pub fn dark() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_dark();
        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: palette.gray(S900),
                surface: palette.gray(S800),
                surface_variant: palette.gray(S700),
                text: palette.gray(S100),
                text_secondary: palette.gray(S400),
                text_disabled: palette.gray(S600),
                primary: palette.blue(S600),
                primary_light: palette.blue(S400),
                primary_dark: palette.blue(S700),
                primary_variant: palette.blue(S800),
                accent: palette.purple(S500),
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

    /// Modern light theme
    pub fn light() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::modern_light();
        Self {
            preference: egui::ThemePreference::Light,
            colors: ThemeColors {
                background: palette.gray(S900),
                surface: palette.gray(S800),
                surface_variant: palette.gray(S700),
                text: palette.gray(S100),
                text_secondary: palette.gray(S400),
                text_disabled: palette.gray(S600),
                primary: ColorPalette::modern_dark().blue(S600),
                primary_light: ColorPalette::modern_dark().blue(S400),
                primary_dark: ColorPalette::modern_dark().blue(S700),
                primary_variant: ColorPalette::modern_dark().blue(S100),
                accent: ColorPalette::modern_dark().purple(S600),
                success: ColorPalette::modern_dark().green(S600),
                warning: ColorPalette::modern_dark().yellow(S600),
                error: ColorPalette::modern_dark().red(S600),
                info: ColorPalette::modern_dark().teal(S600),
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

    /// Nord theme
    pub fn nord() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::nord();
        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: palette.gray(S900),
                surface: palette.gray(S800),
                surface_variant: palette.gray(S700),
                text: palette.gray(S50),
                text_secondary: palette.gray(S200),
                text_disabled: palette.gray(S300),
                primary: palette.blue(S600),
                primary_light: palette.blue(S500),
                primary_dark: palette.blue(S700),
                primary_variant: palette.blue(S800),
                accent: palette.purple(S400),
                success: palette.green(S500),
                warning: palette.yellow(S400),
                error: palette.red(S500),
                info: palette.teal(S400),
                border: palette.gray(S600),
                divider: palette.gray(S700),
                ble_signal_strong: palette.green(S400),
                ble_signal_good: palette.teal(S400),
                ble_signal_weak: palette.yellow(S400),
                ble_signal_very_weak: palette.red(S400),
                ble_connected: palette.green(S500),
                ble_disconnected: palette.gray(S500),
                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// Dracula theme
    pub fn dracula() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
        let palette = ColorPalette::dracula();
        Self {
            preference: egui::ThemePreference::Dark,
            colors: ThemeColors {
                background: palette.gray(S900),
                surface: palette.gray(S800),
                surface_variant: palette.gray(S700),
                text: palette.gray(S50),
                text_secondary: palette.gray(S500),
                text_disabled: palette.gray(S400),
                primary: palette.purple(S400),
                primary_light: palette.purple(S300),
                primary_dark: palette.purple(S600),
                primary_variant: palette.purple(S800),
                accent: palette.red(S400),
                success: palette.green(S400),
                warning: palette.yellow(S400),
                error: palette.red(S500),
                info: palette.blue(S400),
                border: palette.gray(S600),
                divider: palette.gray(S700),
                ble_signal_strong: palette.green(S400),
                ble_signal_good: palette.blue(S400),
                ble_signal_weak: palette.yellow(S400),
                ble_signal_very_weak: palette.red(S400),
                ble_connected: palette.green(S400),
                ble_disconnected: palette.gray(S500),
                floating: None,
                faint_bg: None,
                hovered_bg: None,
            },
        }
    }

    /// ImAuto theme
    pub fn imauto() -> Self {
        use crate::foundation::presets::IMAUTO_THEME;
        IMAUTO_THEME.clone()
    }

    /// Dark Blue theme
    pub fn dark_blue() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
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

    /// Dark Purple theme
    pub fn dark_purple() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
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

    /// Deep Ocean theme
    pub fn deep_ocean() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
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

    /// Midnight Blue theme
    pub fn midnight_blue() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
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

    /// Dark Contrast theme
    pub fn dark_contrast() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
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

    /// One Dark theme
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

    /// Gruvbox Dark theme
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

    /// Tokyo Night theme
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

    /// Light Blue theme
    pub fn light_blue() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
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

    /// Solarized Light theme
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

    /// Light Contrast theme
    pub fn light_contrast() -> Self {
        use crate::foundation::colors::{ColorPalette, Shade::*};
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

    /// Catppuccin theme
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

    /// Gruvbox Light theme
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

    /// GitHub Light theme
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

// =============================================================================
// Style 主题 (推荐)
// =============================================================================

/// 主题名称枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeName {
    /// 现代化暗色主题
    ModernDark,
    /// 北欧风暗色主题
    Nord,
    /// 吸血鬼暗色主题
    Dracula,
    /// 东京夜暗色主题
    TokyoNight,
    /// One Dark 暗色主题
    OneDark,
    /// 纯黑高对比度主题
    DeepBlack,
    /// 赛博朋克霓虹主题
    Cyberpunk,
    /// 矩阵黑客主题
    Matrix,
    /// Monokai Pro 主题
    Monokai,
    /// Ayu Modern 暗色主题
    AyuDark,
    /// 现代化亮色主题
    ModernLight,
    /// GitHub 亮色主题
    GitHubLight,
    /// Solarized 亮色主题
    SolarizedLight,
    /// Catppuccin 亮色主题
    Catppuccin,
    /// Gruvbox 亮色主题
    GruvboxLight,
}

impl ThemeName {
    /// 获取主题显示名称
    pub fn display_name(self) -> &'static str {
        match self {
            Self::ModernDark => "Modern Dark",
            Self::Nord => "Nord",
            Self::Dracula => "Dracula",
            Self::TokyoNight => "Tokyo Night",
            Self::OneDark => "One Dark",
            Self::DeepBlack => "Deep Black",
            Self::Cyberpunk => "Cyberpunk",
            Self::Matrix => "Matrix",
            Self::Monokai => "Monokai",
            Self::AyuDark => "Ayu Dark",
            Self::ModernLight => "Modern Light",
            Self::GitHubLight => "GitHub Light",
            Self::SolarizedLight => "Solarized Light",
            Self::Catppuccin => "Catppuccin",
            Self::GruvboxLight => "Gruvbox Light",
        }
    }

    /// 获取主题描述
    pub fn description(self) -> &'static str {
        match self {
            Self::ModernDark => "现代化暗色主题，通用且专业",
            Self::Nord => "北欧极简风格，冷色调护眼",
            Self::Dracula => "高对比度暗色，色彩丰富",
            Self::TokyoNight => "东京夜意境，渐变蓝色调",
            Self::OneDark => "VS Code 经典主题",
            Self::DeepBlack => "纯黑背景超白文字，极高对比度",
            Self::Cyberpunk => "赛博朋克霓虹粉蓝，未来科技感",
            Self::Matrix => "黑客帝国矩阵绿，科技极客风",
            Self::Monokai => "Monokai Pro 专业暖色调",
            Self::AyuDark => "Ayu Modern 橙蓝对比，现代简约",
            Self::ModernLight => "现代化亮色主题，清新明亮",
            Self::GitHubLight => "GitHub 官方亮色风格",
            Self::SolarizedLight => "Solarized 护眼亮色",
            Self::Catppuccin => "Catppuccin 柔和色调",
            Self::GruvboxLight => "复古暖色调亮色",
        }
    }

    /// 判断是否为暗色主题
    pub fn is_dark(self) -> bool {
        matches!(
            self,
            Self::ModernDark | Self::Nord | Self::Dracula | Self::TokyoNight | Self::OneDark
                | Self::DeepBlack | Self::Cyberpunk | Self::Matrix | Self::Monokai | Self::AyuDark
        )
    }
}

impl std::fmt::Display for ThemeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// 根据主题名称获取完整的 Style
#[must_use]
pub fn style_by_name(name: ThemeName) -> egui::Style {
    match name {
        ThemeName::ModernDark => style::dark::modern_dark(),
        ThemeName::Nord => style::dark::nord(),
        ThemeName::Dracula => style::dark::dracula(),
        ThemeName::TokyoNight => style::dark::tokyo_night(),
        ThemeName::OneDark => style::dark::one_dark(),
        ThemeName::DeepBlack => style::dark::deep_black(),
        ThemeName::Cyberpunk => style::dark::cyberpunk(),
        ThemeName::Matrix => style::dark::matrix(),
        ThemeName::Monokai => style::dark::monokai(),
        ThemeName::AyuDark => style::dark::ayu_dark(),
        ThemeName::ModernLight => style::light::modern_light(),
        ThemeName::GitHubLight => style::light::github_light(),
        ThemeName::SolarizedLight => style::light::solarized_light(),
        ThemeName::Catppuccin => style::light::catppuccin(),
        ThemeName::GruvboxLight => style::light::gruvbox_light(),
    }
}
