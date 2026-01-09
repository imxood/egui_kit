//! Dracula 暗色主题
//!
//! 特点:
//! - 高对比度设计，色彩丰富
//! - 紫色为主色调，视觉效果强烈
//! - 圆角适中 (6px)，阴影强烈
//!
//! 颜色来源: Dracula Color Palette
//! 适用场景: 创意应用、娱乐软件

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Dracula 暗色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: true - 启用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.7 - 弱文本透明度较高
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: DARK_MODE_DEFAULT - 暗色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 深紫黑 (30, 32, 48)
/// - 弱背景填充: 同背景色
/// - 边框: 1px 灰紫 (68, 71, 90)
/// - 文字颜色: 1px 灰 (144, 148, 151)
/// - 圆角: 6px (适中)
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 灰紫 (68, 71, 90)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 米白 (248, 248, 242)
/// - 圆角: 5px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅灰紫 (98, 104, 124)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 主色 (Purple)
/// - 文字颜色: 1.5px 纯白
/// - 圆角: 6px
/// - 扩张: 3.0 (强烈的悬停放大效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 主色 Purple (189, 147, 249) - 高亮填充
/// - 弱背景填充: 同上
/// - 边框: 2px 纯白
/// - 文字颜色: 2px 纯白
/// - 圆角: 5px
/// - 扩张: 2.0 (明显的按下效果)
///
/// ### open: 展开状态
/// - 背景填充: 表面色 (40, 42, 54)
/// - 弱背景填充: 同上
/// - 边框: 1px 灰紫 (98, 104, 124)
/// - 文字颜色: 1px 米白
/// - 圆角: 6px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 灰紫 (68, 71, 90)
/// - 边框: 1px 主色 (Purple)
///
/// ## 颜色配置
/// - `hyperlink_color`: Cyan (139, 233, 253)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 10)
/// - `extreme_bg_color`: 极端背景色 (20, 22, 34) - 更深
/// - `code_bg_color`: 代码块背景 (40, 42, 54)
/// - `warn_fg_color`: Yellow (241, 250, 140)
/// - `error_fg_color`: Red (255, 85, 85)
///
/// ## 窗口样式 (window)
/// - 圆角: 8px (较大)
/// - 阴影: offset=[15,20], blur=25, alpha=120 (强烈的阴影)
/// - 填充色: 背景色
/// - 边框: 1.5px 灰紫
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 6px
///
/// ## 面板样式 (panel)
/// - 填充色: 表面色 (40, 42, 54)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[10,15], blur=15, alpha=100
///
/// ## 其他组件
/// - `resize_corner_size`: 14.0 - 调整角较大
/// - `text_cursor`: 文本光标样式 (2px 主色, 闪烁 0.4s/0.4s 快速闪烁)
/// - `clip_rect_margin`: 4.0 - 剪裁边距
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: true - 滑块尾部填充 (Dracula 特色)
/// - `handle_shape`: Rect(1.0) - 滑块句柄正方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.5 - 禁用状态透明度
#[must_use]
pub fn dracula() -> Style {
    // Dracula 颜色定义
    let background = Color32::from_rgb(30, 32, 48);      // Background - 深紫黑
    let surface = Color32::from_rgb(40, 42, 54);         // Current Line - 稍浅
    let primary = Color32::from_rgb(189, 147, 249);      // Purple - 紫色
    let warning = Color32::from_rgb(241, 250, 140);      // Yellow - 黄色
    let error = Color32::from_rgb(255, 85, 85);          // Red - 红色
    let hyperlink = Color32::from_rgb(139, 233, 253);    // Cyan - 青色

    Style {
        visuals: Visuals {
            // === 基础模式设置 ===
            dark_mode: true, // 启用暗色模式
            override_text_color: None,
            weak_text_alpha: 0.7,
            weak_text_color: None,
            text_alpha_from_coverage: AlphaFromCoverage::DARK_MODE_DEFAULT,

            // === 组件状态视觉 ===
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background,
                    weak_bg_fill: background,
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(68, 71, 90)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(144, 148, 151)),
                    corner_radius: CornerRadius::same(6),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(68, 71, 90),
                    weak_bg_fill: Color32::from_rgb(68, 71, 90),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(248, 248, 242)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgb(98, 104, 124),
                    weak_bg_fill: Color32::from_rgb(98, 104, 124),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(255, 255, 255)),
                    corner_radius: CornerRadius::same(6),
                    expansion: 3.0, // 强烈的悬停效果
                },
                active: WidgetVisuals {
                    bg_fill: primary, // 高亮填充
                    weak_bg_fill: primary,
                    bg_stroke: Stroke::new(2.0, Color32::WHITE),
                    fg_stroke: Stroke::new(2.0, Color32::WHITE),
                    corner_radius: CornerRadius::same(5),
                    expansion: 2.0,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(98, 104, 124)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(248, 248, 242)),
                    corner_radius: CornerRadius::same(6),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgb(68, 71, 90),
                stroke: Stroke::new(1.0, primary),
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(10),
            extreme_bg_color: Color32::from_rgb(20, 22, 34),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(40, 42, 54),
            warn_fg_color: warning,
            error_fg_color: error,

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(8),
            window_shadow: Shadow {
                offset: [15, 20],
                blur: 25,
                spread: 0,
                color: Color32::from_black_alpha(120),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.5, Color32::from_rgb(68, 71, 90)),
            window_highlight_topmost: true,

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(6),

            // === 面板样式 ===
            panel_fill: surface,

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [10, 15],
                blur: 15,
                spread: 0,
                color: Color32::from_black_alpha(100),
            },

            // === 其他组件样式 ===
            resize_corner_size: 14.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, primary),
                preview: false,
                blink: true,
                on_duration: 0.4,
                off_duration: 0.4,
            },
            clip_rect_margin: 4.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: true, // 滑块尾部填充
            handle_shape: HandleShape::Rect { aspect_ratio: 1.0 }, // 正方形句柄
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.5,
        },
        ..Default::default()
    }
}
