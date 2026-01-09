//! Gruvbox Light 亮色主题
//!
//! 特点:
//! - 复古暖色调，模仿老式终端
//! - 温暖舒适的视觉效果
//! - 高对比度，清晰易读
//!
//! 颜色来源: Gruvbox Light
//! 适用场景: 复古风格应用、代码编辑器

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Gruvbox 亮色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: false - 禁用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.6 - 弱文本透明度
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: LIGHT_MODE_DEFAULT - 亮色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 米黄 (251, 241, 199)
/// - 弱背景填充: 浅米黄 (250, 238, 195)
/// - 边框: 1.5px 棕灰 (130, 120, 105)
/// - 文字颜色: 1.5px 深褐 (40, 35, 32)
/// - 圆角: 5px (适中)
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 浅米黄 (235, 219, 178)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 深褐 (45, 40, 38)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅青 (235, 240, 220)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 主色 (青色)
/// - 文字颜色: 1.5px 纯黑
/// - 圆角: 5px
/// - 扩张: 2.5 (明显的悬停效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 主色浅 (69, 133, 136, 25)
/// - 弱背景填充: 同上
/// - 边框: 2px 主色 (青色)
/// - 文字颜色: 2px 纯黑
/// - 圆角: 4px
/// - 扩张: 1.0
///
/// ### open: 展开状态
/// - 背景填充: 表面色 (248, 240, 215)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 棕灰 (140, 130, 115)
/// - 文字颜色: 1px 深褐 (45, 40, 38)
/// - 圆角: 5px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 主色浅 (69, 133, 136, 30)
/// - 边框: 1.5px 主色 (青色)
///
/// ## 颜色配置
/// - `hyperlink_color`: 深青 (50, 100, 105)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 5)
/// - `extreme_bg_color`: 极端背景色 (255, 250, 235) - 米白
/// - `code_bg_color`: 代码块背景 (245, 235, 210)
/// - `warn_fg_color`: 深橙黄 (175, 120, 25)
/// - `error_fg_color`: 鲜红 (175, 30, 25)
///
/// ## 窗口样式 (window)
/// - 圆角: 6px
/// - 阴影: offset=[8,12], blur=20, alpha=40 (清晰柔和)
/// - 填充色: 米黄
/// - 边框: 1.5px 棕灰 (145, 135, 120)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 5px
///
/// ## 面板样式 (panel)
/// - 填充色: 表面色 (248, 240, 215)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[5,8], blur=12, alpha=35
///
/// ## 其他组件
/// - `resize_corner_size`: 10.0
/// - `text_cursor`: 文本光标样式 (2.5px 深青, 闪烁 0.4s/0.5s)
/// - `clip_rect_margin`: 3.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: true - 滑块尾部填充
/// - `handle_shape`: Rect(0.75) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.45 - 禁用状态透明度
#[must_use]
pub fn gruvbox_light() -> Style {
    // Gruvbox Light 颜色定义 (高对比度)
    let background = Color32::from_rgb(251, 241, 199);   // Light - 米黄
    let surface = Color32::from_rgb(248, 240, 215);      // Surface - 表面色
    let primary = Color32::from_rgb(50, 100, 105);       // 深青 - 高对比度
    let warning = Color32::from_rgb(175, 120, 25);       // 深橙黄
    let error = Color32::from_rgb(175, 30, 25);          // 鲜红
    let hyperlink = Color32::from_rgb(50, 100, 105);     // 深青

    Style {
        visuals: Visuals {
            dark_mode: false,
            override_text_color: None,
            weak_text_alpha: 0.6,
            weak_text_color: None,
            text_alpha_from_coverage: AlphaFromCoverage::LIGHT_MODE_DEFAULT,

            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background,
                    weak_bg_fill: Color32::from_rgb(250, 238, 195),
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(130, 120, 105)),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(40, 35, 32)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(235, 219, 178),
                    weak_bg_fill: Color32::from_rgb(235, 219, 178),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(45, 40, 38)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(69, 133, 136, 18),
                    weak_bg_fill: Color32::from_rgba_premultiplied(69, 133, 136, 18),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(1.5, Color32::BLACK),
                    corner_radius: CornerRadius::same(5),
                    expansion: 2.5,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(69, 133, 136, 28),
                    weak_bg_fill: Color32::from_rgba_premultiplied(69, 133, 136, 28),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::BLACK),
                    corner_radius: CornerRadius::same(4),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(140, 130, 115)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(45, 40, 38)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
            },

            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(50, 100, 105, 30),
                stroke: Stroke::new(1.5, primary),
            },

            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(5),
            extreme_bg_color: Color32::from_rgb(255, 250, 235),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(245, 235, 210),
            warn_fg_color: warning,
            error_fg_color: error,

            window_corner_radius: CornerRadius::same(6),
            window_shadow: Shadow {
                offset: [8, 12],
                blur: 20,
                spread: 0,
                color: Color32::from_black_alpha(40),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.5, Color32::from_rgb(145, 135, 120)),
            window_highlight_topmost: true,
            menu_corner_radius: CornerRadius::same(5),
            panel_fill: surface,
            popup_shadow: Shadow {
                offset: [5, 8],
                blur: 12,
                spread: 0,
                color: Color32::from_black_alpha(35),
            },
            resize_corner_size: 10.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.5, primary),
                preview: false,
                blink: true,
                on_duration: 0.4,
                off_duration: 0.5,
            },
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: true,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.75 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.45,
        },
        ..Default::default()
    }
}
