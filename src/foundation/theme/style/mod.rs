//! 主题样式定义
//!
//! 基于 [`egui::Style`] 的完整主题定义.
//! 每种主题都是精心调教的完整 Style 配置.

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// 主题样式类型别名
pub type ThemeStyle = Style;

/// 深色主题模块
pub mod dark;
/// 亮色主题模块
pub mod light;

/// 所有深色主题列表
pub const DARK_THEMES: &[(&str, fn() -> ThemeStyle)] = &[
    ("Modern Dark", dark::modern_dark),
    ("Nord", dark::nord),
    ("Dracula", dark::dracula),
    ("Tokyo Night", dark::tokyo_night),
    ("One Dark", dark::one_dark),
    ("Deep Black", dark::deep_black),
    ("Cyberpunk", dark::cyberpunk),
    ("Matrix", dark::matrix),
    ("Monokai", dark::monokai),
    ("Ayu Dark", dark::ayu_dark),
];

/// 所有亮色主题列表
pub const LIGHT_THEMES: &[(&str, fn() -> ThemeStyle)] = &[
    ("Modern Light", light::modern_light),
    ("GitHub Light", light::github_light),
    ("Solarized Light", light::solarized_light),
    ("Catppuccin", light::catppuccin),
    ("Gruvbox Light", light::gruvbox_light),
];

/// 所有主题列表 (暗色 + 亮色)
pub const ALL_THEMES: &[(&str, fn() -> ThemeStyle, bool)] = &[
    // Dark themes (is_dark = true)
    ("Modern Dark", dark::modern_dark, true),
    ("Nord", dark::nord, true),
    ("Dracula", dark::dracula, true),
    ("Tokyo Night", dark::tokyo_night, true),
    ("One Dark", dark::one_dark, true),
    ("Deep Black", dark::deep_black, true),
    ("Cyberpunk", dark::cyberpunk, true),
    ("Matrix", dark::matrix, true),
    ("Monokai", dark::monokai, true),
    ("Ayu Dark", dark::ayu_dark, true),
    // Light themes (is_dark = false)
    ("Modern Light", light::modern_light, false),
    ("GitHub Light", light::github_light, false),
    ("Solarized Light", light::solarized_light, false),
    ("Catppuccin", light::catppuccin, false),
    ("Gruvbox Light", light::gruvbox_light, false),
];

// =============================================================================
// Style 参数中文释义参考 (开发文档)
// =============================================================================
//
// ## 视觉效果 (Visuals)
// - dark_mode: 深色/亮色模式标志
// - widgets: 组件状态视觉
//   - noninteractive: 不可交互状态 (标签、分割线)
//   - inactive: 未激活状态 (按钮默认)
//   - hovered: 悬停状态
//   - active: 按下/拖拽状态
//   - open: 打开状态 (菜单、下拉框)
//   每种状态包含:
//   - bg_fill: 强制背景色
//   - weak_bg_fill: 可选背景色
//   - bg_stroke: 背景边框
//   - fg_stroke: 前景/文字颜色
//   - corner_radius: 圆角半径
//   - expansion: 扩展量 (悬停时放大效果)
//
// - selection: 文本选择样式
//   - bg_fill: 选择背景色
//   - stroke: 选择边框色
//
// - window_corner_radius: 窗口圆角
// - window_shadow: 窗口阴影
// - window_fill: 窗口背景
// - window_stroke: 窗口边框
//
// - panel_fill: 面板背景
// - menu_corner_radius: 菜单圆角
// - popup_shadow: 弹出层阴影
//
// - hyperlink_color: 超链接
// - warn_fg_color: 警告色
// - error_fg_color: 错误色
// - code_bg_color: 代码背景
//
// - text_cursor: 文本光标样式
// - handle_shape: 滑块手柄形状
// - disabled_alpha: 禁用状态透明度
