// Theme presets for quick selection

use super::theme::ThemeConfig;

/// Available theme presets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ThemePreset {
    // ===== 暗色主题 =====

    /// Modern dark theme (default) - inspired by GitHub Dark & VS Code
    Dark,

    /// Dark blue theme - 深邃的蓝黑色调，适合长时间编码
    DarkBlue,

    /// Dark purple theme - 优雅的紫黑色调，神秘而专业
    DarkPurple,

    /// Deep ocean theme - 深海般的蓝黑渐变，沉稳大气
    DeepOcean,

    /// Midnight blue theme - 午夜蓝黑，柔和而不刺眼
    MidnightBlue,

    /// High contrast dark - 高对比度暗色，文字极度清晰
    DarkContrast,

    /// One Dark theme - VS Code经典暗色主题
    OneDark,

    /// Gruvbox Dark - 复古暖色暗色主题
    GruvboxDark,

    /// Tokyo Night theme - 东京夜景，流行的开发者主题
    TokyoNight,

    /// Nord theme - minimal and elegant
    Nord,

    /// Dracula theme - vibrant and eye-catching
    Dracula,

    /// ImAuto theme - professional dark theme with warm accents
    ImAuto,

    // ===== 亮色主题 =====

    /// Modern light theme
    Light,

    /// Light blue theme - 清爽的蓝白色调
    LightBlue,

    /// Solarized Light - 经典护眼亮色主题
    SolarizedLight,

    /// High contrast light - 高对比度亮色
    LightContrast,

    /// Catppuccin Latte - 柔和的拿铁色调
    Catppuccin,

    /// Gruvbox Light - 复古暖色亮色主题
    GruvboxLight,

    /// GitHub Light - GitHub经典亮色
    GitHubLight,
}

impl ThemePreset {
    /// Get all available presets
    pub const fn all() -> &'static [ThemePreset] {
        &[
            // 暗色主题
            Self::Dark,
            Self::DarkBlue,
            Self::DarkPurple,
            Self::DeepOcean,
            Self::MidnightBlue,
            Self::DarkContrast,
            Self::OneDark,
            Self::GruvboxDark,
            Self::TokyoNight,
            Self::Nord,
            Self::Dracula,
            Self::ImAuto,
            // 亮色主题
            Self::Light,
            Self::LightBlue,
            Self::SolarizedLight,
            Self::LightContrast,
            Self::Catppuccin,
            Self::GruvboxLight,
            Self::GitHubLight,
        ]
    }

    /// Get preset name for UI display
    pub const fn name(&self) -> &'static str {
        match self {
            // 暗色主题
            Self::Dark => "Modern Dark",
            Self::DarkBlue => "Dark Blue",
            Self::DarkPurple => "Dark Purple",
            Self::DeepOcean => "Deep Ocean",
            Self::MidnightBlue => "Midnight Blue",
            Self::DarkContrast => "High Contrast Dark",
            Self::OneDark => "One Dark",
            Self::GruvboxDark => "Gruvbox Dark",
            Self::TokyoNight => "Tokyo Night",
            Self::Nord => "Nord",
            Self::Dracula => "Dracula",
            Self::ImAuto => "ImAuto Professional",
            // 亮色主题
            Self::Light => "Modern Light",
            Self::LightBlue => "Light Blue",
            Self::SolarizedLight => "Solarized Light",
            Self::LightContrast => "High Contrast Light",
            Self::Catppuccin => "Catppuccin Latte",
            Self::GruvboxLight => "Gruvbox Light",
            Self::GitHubLight => "GitHub Light",
        }
    }

    /// Get preset description
    pub const fn description(&self) -> &'static str {
        match self {
            // 暗色主题
            Self::Dark => "现代暗色，灵感来自GitHub Dark和VS Code，平衡的蓝灰色调",
            Self::DarkBlue => "深邃蓝黑，适合长时间编码，舒适的蓝色主调减少眼疲劳",
            Self::DarkPurple => "优雅紫黑，神秘而专业，紫色点缀提升视觉层次",
            Self::DeepOcean => "深海蓝黑，沉稳大气，蓝色渐变营造深度感",
            Self::MidnightBlue => "午夜蓝黑，柔和不刺眼，暗蓝色背景更护眼",
            Self::DarkContrast => "高对比度暗色，文字极度清晰，适合弱视用户",
            Self::OneDark => "VS Code经典暗色，柔和的紫灰色调，广受开发者喜爱",
            Self::GruvboxDark => "复古暖色暗色，棕褐色调温暖舒适，长时间使用不疲劳",
            Self::TokyoNight => "东京夜景，流行的开发者主题，蓝紫色调时尚现代",
            Self::Nord => "北欧极简，冰川般的蓝灰色调，清爽优雅",
            Self::Dracula => "德古拉紫，鲜艳的紫粉色，个性张扬充满活力",
            Self::ImAuto => "专业深蓝，深蓝灰背景配暖黄点缀，商务专业",
            // 亮色主题
            Self::Light => "现代亮色，柔和的灰白色调，阅读舒适",
            Self::LightBlue => "清爽蓝白，天空般的蓝白色调，清新明快",
            Self::SolarizedLight => "Solarized经典护眼亮色，精心调配的暖色调",
            Self::LightContrast => "高对比度亮色，黑白分明，文字极度清晰",
            Self::Catppuccin => "拿铁柔和，温暖的奶油色调，柔和不刺眼",
            Self::GruvboxLight => "复古暖色亮色，米黄色调温暖舒适",
            Self::GitHubLight => "GitHub经典亮色，清爽的白底灰边，简洁专业",
        }
    }
}

impl Default for ThemePreset {
    fn default() -> Self {
        Self::Dark
    }
}

// Convenience constants for quick access
pub const DARK_THEME: ThemeConfig = ThemeConfig {
    preference: egui::ThemePreference::Dark,
    colors: super::theme::ThemeColors {
        background: egui::Color32::from_rgb(15, 23, 42),
        surface: egui::Color32::from_rgb(30, 41, 59),
        surface_variant: egui::Color32::from_rgb(51, 65, 85),
        text: egui::Color32::from_rgb(241, 245, 249),
        text_secondary: egui::Color32::from_rgb(148, 163, 184),
        text_disabled: egui::Color32::from_rgb(71, 85, 105),
        primary: egui::Color32::from_rgb(37, 99, 235),
        primary_light: egui::Color32::from_rgb(96, 165, 250),
        primary_dark: egui::Color32::from_rgb(29, 78, 216),
        primary_variant: egui::Color32::from_rgb(30, 64, 175),
        accent: egui::Color32::from_rgb(168, 85, 247),
        success: egui::Color32::from_rgb(34, 197, 94),
        warning: egui::Color32::from_rgb(234, 179, 8),
        error: egui::Color32::from_rgb(239, 68, 68),
        info: egui::Color32::from_rgb(20, 184, 166),
        border: egui::Color32::from_rgb(71, 85, 105),
        divider: egui::Color32::from_rgb(51, 65, 85),
        ble_signal_strong: egui::Color32::from_rgb(74, 222, 128),
        ble_signal_good: egui::Color32::from_rgb(45, 212, 191),
        ble_signal_weak: egui::Color32::from_rgb(234, 179, 8),
        ble_signal_very_weak: egui::Color32::from_rgb(248, 113, 113),
        ble_connected: egui::Color32::from_rgb(34, 197, 94),
        ble_disconnected: egui::Color32::from_rgb(100, 116, 139),
        floating: None,
        faint_bg: None,
        hovered_bg: None,
    },
};

pub const LIGHT_THEME: ThemeConfig = ThemeConfig {
    preference: egui::ThemePreference::Light,
    colors: super::theme::ThemeColors {
        background: egui::Color32::from_rgb(248, 250, 252),
        surface: egui::Color32::from_rgb(241, 245, 249),
        surface_variant: egui::Color32::from_rgb(226, 232, 240),
        text: egui::Color32::from_rgb(15, 23, 42),
        text_secondary: egui::Color32::from_rgb(100, 116, 139),
        text_disabled: egui::Color32::from_rgb(148, 163, 184),
        primary: egui::Color32::from_rgb(37, 99, 235),
        primary_light: egui::Color32::from_rgb(96, 165, 250),
        primary_dark: egui::Color32::from_rgb(29, 78, 216),
        primary_variant: egui::Color32::from_rgb(219, 234, 254),
        accent: egui::Color32::from_rgb(147, 51, 234),
        success: egui::Color32::from_rgb(22, 163, 74),
        warning: egui::Color32::from_rgb(202, 138, 4),
        error: egui::Color32::from_rgb(220, 38, 38),
        info: egui::Color32::from_rgb(13, 148, 136),
        border: egui::Color32::from_rgb(148, 163, 184),
        divider: egui::Color32::from_rgb(203, 213, 225),
        ble_signal_strong: egui::Color32::from_rgb(34, 197, 94),
        ble_signal_good: egui::Color32::from_rgb(20, 184, 166),
        ble_signal_weak: egui::Color32::from_rgb(202, 138, 4),
        ble_signal_very_weak: egui::Color32::from_rgb(239, 68, 68),
        ble_connected: egui::Color32::from_rgb(22, 163, 74),
        ble_disconnected: egui::Color32::from_rgb(100, 116, 139),
        floating: None,
        faint_bg: None,
        hovered_bg: None,
    },
};

pub const NORD_THEME: ThemeConfig = ThemeConfig {
    preference: egui::ThemePreference::Dark,
    colors: super::theme::ThemeColors {
        background: egui::Color32::from_rgb(46, 52, 64),
        surface: egui::Color32::from_rgb(59, 66, 82),
        surface_variant: egui::Color32::from_rgb(67, 76, 94),
        text: egui::Color32::from_rgb(236, 239, 244),
        text_secondary: egui::Color32::from_rgb(216, 222, 233),
        text_disabled: egui::Color32::from_rgb(143, 157, 180),
        primary: egui::Color32::from_rgb(94, 129, 172),
        primary_light: egui::Color32::from_rgb(129, 161, 193),
        primary_dark: egui::Color32::from_rgb(81, 119, 162),
        primary_variant: egui::Color32::from_rgb(70, 100, 145),
        accent: egui::Color32::from_rgb(180, 142, 173),
        success: egui::Color32::from_rgb(163, 190, 140),
        warning: egui::Color32::from_rgb(235, 203, 139),
        error: egui::Color32::from_rgb(191, 97, 106),
        info: egui::Color32::from_rgb(136, 192, 208),
        border: egui::Color32::from_rgb(76, 86, 106),
        divider: egui::Color32::from_rgb(67, 76, 94),
        ble_signal_strong: egui::Color32::from_rgb(163, 190, 140),
        ble_signal_good: egui::Color32::from_rgb(136, 192, 208),
        ble_signal_weak: egui::Color32::from_rgb(235, 203, 139),
        ble_signal_very_weak: egui::Color32::from_rgb(208, 135, 112),
        ble_connected: egui::Color32::from_rgb(163, 190, 140),
        ble_disconnected: egui::Color32::from_rgb(94, 129, 172),
        floating: None,
        faint_bg: None,
        hovered_bg: None,
    },
};

pub const DRACULA_THEME: ThemeConfig = ThemeConfig {
    preference: egui::ThemePreference::Dark,
    colors: super::theme::ThemeColors {
        background: egui::Color32::from_rgb(30, 32, 44),
        surface: egui::Color32::from_rgb(40, 42, 54),
        surface_variant: egui::Color32::from_rgb(68, 71, 90),
        text: egui::Color32::from_rgb(248, 248, 242),
        text_secondary: egui::Color32::from_rgb(98, 114, 164),
        text_disabled: egui::Color32::from_rgb(138, 138, 138),
        primary: egui::Color32::from_rgb(189, 147, 249),
        primary_light: egui::Color32::from_rgb(216, 180, 254),
        primary_dark: egui::Color32::from_rgb(147, 96, 207),
        primary_variant: egui::Color32::from_rgb(107, 33, 168),
        accent: egui::Color32::from_rgb(255, 121, 198),
        success: egui::Color32::from_rgb(80, 250, 123),
        warning: egui::Color32::from_rgb(241, 250, 140),
        error: egui::Color32::from_rgb(255, 85, 85),
        info: egui::Color32::from_rgb(139, 233, 253),
        border: egui::Color32::from_rgb(98, 114, 164),
        divider: egui::Color32::from_rgb(68, 71, 90),
        ble_signal_strong: egui::Color32::from_rgb(80, 250, 123),
        ble_signal_good: egui::Color32::from_rgb(139, 233, 253),
        ble_signal_weak: egui::Color32::from_rgb(241, 250, 140),
        ble_signal_very_weak: egui::Color32::from_rgb(255, 121, 198),
        ble_connected: egui::Color32::from_rgb(80, 250, 123),
        ble_disconnected: egui::Color32::from_rgb(98, 114, 164),
        floating: None,
        faint_bg: None,
        hovered_bg: None,
    },
};

pub const IMAUTO_THEME: ThemeConfig = ThemeConfig {
    preference: egui::ThemePreference::Dark,
    colors: super::theme::ThemeColors {
        // Backgrounds - from im_auto2 design tokens (完全匹配)
        background: egui::Color32::from_rgb(13, 16, 17),        // gray(S100) = #0d1011 - panel_bg_color
        surface: egui::Color32::from_rgb(72, 84, 90),           // inactive.bg_fill (gray-blue)
        surface_variant: egui::Color32::from_rgb(58, 64, 93),   // inactive.weak_bg_fill (deep blue)

        // Text colors (完全匹配原始主题，去除 -10% 调整)
        text: egui::Color32::from_rgb(255, 255, 255),           // noninteractive.fg_stroke (纯白)
        text_secondary: egui::Color32::from_rgb(223, 225, 235), // inactive.fg_stroke (亮灰白)
        text_disabled: egui::Color32::from_rgb(150, 160, 180),  // muted gray (保持原有)

        // Primary colors (Blue tones from im_auto2，完全匹配)
        primary: egui::Color32::from_rgb(0, 88, 255),           // hovered.bg_fill (纯蓝)
        primary_light: egui::Color32::from_rgb(80, 150, 255),   // active.bg_fill (亮蓝)
        primary_dark: egui::Color32::from_rgb(0, 70, 200),      // darker blue for pressed state (保持原有)
        primary_variant: egui::Color32::from_rgb(173, 184, 255), // selection.stroke (淡蓝紫，完全匹配)

        // Accent (warm yellow from hover/active strokes - im_auto2's signature，完全匹配)
        accent: egui::Color32::from_rgb(255, 215, 153),         // hovered.fg_stroke (暖黄)

        // Semantic colors (完全匹配)
        success: egui::Color32::from_rgb(80, 250, 123),         // vibrant green (保持原有)
        warning: egui::Color32::from_rgb(255, 122, 12),         // warn_fg_color (0xFF7A0C，完全匹配)
        error: egui::Color32::from_rgb(171, 1, 22),             // error_fg_color (0xAB0116，完全匹配)
        info: egui::Color32::from_rgb(139, 233, 253),           // cyan (保持原有)

        // Borders (完全匹配)
        border: egui::Color32::from_rgb(73, 81, 140),           // noninteractive.bg_stroke (紫蓝)
        divider: egui::Color32::from_rgb(50, 56, 70),           // subtle divider (保持原有)

        // BLE-specific (使用语义颜色，保持原有)
        ble_signal_strong: egui::Color32::from_rgb(80, 250, 123),
        ble_signal_good: egui::Color32::from_rgb(139, 233, 253),
        ble_signal_weak: egui::Color32::from_rgb(255, 122, 12),
        ble_signal_very_weak: egui::Color32::from_rgb(255, 85, 85),
        ble_connected: egui::Color32::from_rgb(80, 250, 123),
        ble_disconnected: egui::Color32::from_rgb(100, 116, 139),

        // ============================================================================
        // ImAuto-specific intermediate colors (用于精确控制层次感) - 完全匹配 design_tokens
        // ============================================================================
        floating: Some(egui::Color32::from_rgb(38, 43, 46)),     // gray(S250) = #262b2e - window_fill (tooltips, menus)
        faint_bg: Some(egui::Color32::from_rgb(20, 24, 25)),     // gray(S150) = #141819 - faint_bg_color
        hovered_bg: Some(egui::Color32::from_rgb(0, 54, 146)),   // blue(S325) = #003692 - open.bg_fill/weak_bg_fill
    },
};
