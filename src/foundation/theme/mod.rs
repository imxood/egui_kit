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

use serde::{Deserialize, Serialize};

// =============================================================================
// Style 主题 (推荐)
// =============================================================================

/// 主题名称枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeName {
    /// 现代化暗色主题
    ModernDark,
    /// 北欧风暗色主题
    Nord,
    /// 德古拉暗色主题
    Dracula,
    /// 东京夜景主题
    TokyoNight,
    /// One Dark 主题
    OneDark,
    /// 纯黑主题
    DeepBlack,
    /// 赛博朋克主题
    Cyberpunk,
    /// 矩阵主题
    Matrix,
    /// Monokai 主题
    Monokai,
    /// Ayu Dark 主题
    AyuDark,
    /// 现代化亮色主题
    ModernLight,
    /// GitHub 亮色主题
    GitHubLight,
    /// Solarized 亮色主题
    SolarizedLight,
    /// Catppuccin 主题
    Catppuccin,
    /// Gruvbox 亮色主题
    GruvboxLight,
}

impl ThemeName {
    /// Get display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            ThemeName::ModernDark => "Modern Dark",
            ThemeName::Nord => "Nord",
            ThemeName::Dracula => "Dracula",
            ThemeName::TokyoNight => "Tokyo Night",
            ThemeName::OneDark => "One Dark",
            ThemeName::DeepBlack => "Deep Black",
            ThemeName::Cyberpunk => "Cyberpunk",
            ThemeName::Matrix => "Matrix",
            ThemeName::Monokai => "Monokai",
            ThemeName::AyuDark => "Ayu Dark",
            ThemeName::ModernLight => "Modern Light",
            ThemeName::GitHubLight => "GitHub Light",
            ThemeName::SolarizedLight => "Solarized Light",
            ThemeName::Catppuccin => "Catppuccin",
            ThemeName::GruvboxLight => "Gruvbox Light",
        }
    }

    /// Check if theme is dark mode
    pub fn is_dark(&self) -> bool {
        match self {
            ThemeName::ModernDark | ThemeName::Nord | ThemeName::Dracula | ThemeName::TokyoNight
            | ThemeName::OneDark | ThemeName::DeepBlack | ThemeName::Cyberpunk | ThemeName::Matrix
            | ThemeName::Monokai | ThemeName::AyuDark => true,
            ThemeName::ModernLight | ThemeName::GitHubLight | ThemeName::SolarizedLight
            | ThemeName::Catppuccin | ThemeName::GruvboxLight => false,
        }
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
