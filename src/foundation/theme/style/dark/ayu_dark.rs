//! Ayu Dark 暗色主题
//!
//! 特点:
//! - 现代简约风格, 橙蓝对比色
//! - 高对比度设计, 清晰易读
//! - 圆角适中 (4px), 视觉舒适
//!
//! 颜色来源: Ayu Theme Color Palette
//! 适用场景: 通用应用、编辑器、现代Web风格应用

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Ayu Dark 暗色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: true - 启用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.75 - 弱文本透明度
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: DARK_MODE_DEFAULT - 暗色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 深蓝灰 (20, 23, 30)
/// - 弱背景填充: 同背景色
/// - 边框: 1px 灰蓝 (70, 80, 100)
/// - 文字颜色: 1px 灰白 (220, 225, 235)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 蓝灰 (30, 35, 45)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 灰白 (230, 235, 245)
/// - 圆角: 3px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 橙金 (255, 150, 50, 40)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 橙金 (255, 150, 50)
/// - 文字颜色: 1.5px 橙白 (255, 220, 180)
/// - 圆角: 4px
/// - 扩张: 2.5
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 橙金 (255, 150, 50, 75)
/// - 弱背景填充: 同上
/// - 边框: 2px 橙金 (255, 150, 50)
/// - 文字颜色: 2px 橙白 (255, 230, 200)
/// - 圆角: 3px
/// - 扩张: 1.5
///
/// ### open: 展开状态
/// - 背景填充: 深蓝灰 (25, 28, 35)
/// - 弱背景填充: 同上
/// - 边框: 1px 橙金 (255, 150, 50)
/// - 文字颜色: 1px 橙白 (255, 220, 180)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 霓虹蓝 (60, 160, 255, 45)
/// - 边框: 1px 霓虹蓝 (60, 160, 255)
///
/// ## 颜色配置
/// - `hyperlink_color`: 霓虹蓝 (60, 160, 255)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 6)
/// - `extreme_bg_color`: 极端背景色 (深蓝灰 15)
/// - `code_bg_color`: 代码块背景 (35, 40, 50)
/// - `warn_fg_color`: 金橙 (255, 180, 50)
/// - `error_fg_color`: 鲜红 (255, 80, 80)
///
/// ## 窗口样式 (window)
/// - 圆角: 4px
/// - 阴影: offset=[6,10], blur=20, alpha=75
/// - 填充色: 深蓝灰
/// - 边框: 1px 橙金 (255, 150, 50)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 4px
///
/// ## 面板样式 (panel)
/// - 填充色: 蓝灰 (28, 32, 40)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[5,10], blur=18, alpha=65
///
/// ## 其他组件
/// - `resize_corner_size`: 10.0
/// - `text_cursor`: 文本光标样式 (2px 霓虹蓝, 闪烁 0.4s/0.45s)
/// - `clip_rect_margin`: 3.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: true - 滑块尾部填充
/// - `handle_shape`: Rect(0.8) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.55 - 禁用状态透明度
#[must_use]
pub fn ayu_dark() -> Style {
    // Ayu Dark 颜色定义
    let background = Color32::from_rgb(20, 23, 30);      // 深蓝灰
    let surface = Color32::from_rgb(28, 32, 40);        // 蓝灰表面
    let primary = Color32::from_rgb(255, 150, 50);      // Orange Gold - 橙金
    let secondary = Color32::from_rgb(60, 160, 255);    // Neon Blue - 霓虹蓝
    let warning = Color32::from_rgb(255, 180, 50);      // Golden Orange - 金橙
    let error = Color32::from_rgb(255, 80, 80);         // Bright Red - 鲜红
    let hyperlink = Color32::from_rgb(60, 160, 255);    // Neon Blue - 霓虹蓝

    Style {
        visuals: Visuals {
            // === 基础模式设置 ===
            dark_mode: true, // 启用暗色模式
            override_text_color: None,
            weak_text_alpha: 0.75,
            weak_text_color: None,
            text_alpha_from_coverage: AlphaFromCoverage::DARK_MODE_DEFAULT,

            // === 组件状态视觉 ===
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background,
                    weak_bg_fill: background,
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(70, 80, 100)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(220, 225, 235)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(30, 35, 45),
                    weak_bg_fill: Color32::from_rgb(30, 35, 45),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(230, 235, 245)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(255, 150, 50, 40),
                    weak_bg_fill: Color32::from_rgba_premultiplied(255, 150, 50, 40),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(255, 220, 180)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 2.5,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(255, 150, 50, 75),
                    weak_bg_fill: Color32::from_rgba_premultiplied(255, 150, 50, 75),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::from_rgb(255, 230, 200)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 1.5,
                },
                open: WidgetVisuals {
                    bg_fill: Color32::from_rgb(25, 28, 35),
                    weak_bg_fill: Color32::from_rgb(25, 28, 35),
                    bg_stroke: Stroke::new(1.0, primary),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 220, 180)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(60, 160, 255, 45),
                stroke: Stroke::new(1.0, secondary),
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(6),
            extreme_bg_color: Color32::from_rgb(15, 17, 22),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(35, 40, 50),
            warn_fg_color: warning,
            error_fg_color: error,

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(4),
            window_shadow: Shadow {
                offset: [6, 10],
                blur: 20,
                spread: 0,
                color: Color32::from_black_alpha(75),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.0, primary),
            window_highlight_topmost: true,

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(4),

            // === 面板样式 ===
            panel_fill: surface,

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [5, 10],
                blur: 18,
                spread: 0,
                color: Color32::from_black_alpha(65),
            },

            // === 其他组件样式 ===
            resize_corner_size: 10.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, secondary),
                preview: false,
                blink: true,
                on_duration: 0.4,
                off_duration: 0.45,
            },
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: true,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.8 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.55,
        },
        ..Default::default()
    }
}
