//! Modern Light 亮色主题
//!
//! 特点:
//! - 清新明亮的现代设计
//! - 圆角适中 (6px)，柔和阴影
//! - 高对比度，清晰易读
//!
//! 适用场景: 通用应用、办公软件

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// 现代化亮色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: false - 禁用暗色模式 (亮色模式)
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.55 - 弱文本透明度 (较高对比度)
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: LIGHT_MODE_DEFAULT - 亮色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 纯白 (255)
/// - 弱背景填充: 浅灰 (250)
/// - 边框: 1.5px 深灰 (120)
/// - 文字颜色: 1.5px 近黑 (30)
/// - 圆角: 5px
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 浅灰 (235)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 深灰 (50)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅蓝灰 (220, 235, 255)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 主色 (蓝色)
/// - 文字颜色: 1.5px 纯黑
/// - 圆角: 5px
/// - 扩张: 2.5 (明显的悬停放大效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 主色浅 (66, 133, 244, 30)
/// - 弱背景填充: 同上
/// - 边框: 2px 主色 (蓝色)
/// - 文字颜色: 2px 纯黑
/// - 圆角: 4px
/// - 扩张: 1.0
///
/// ### open: 展开状态
/// - 背景填充: 纯白 (255) - 表面色
/// - 弱背景填充: 灰 (245)
/// - 边框: 1.5px 深灰 (150)
/// - 文字颜色: 1px 深灰 (40)
/// - 圆角: 5px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 主色浅 (66, 133, 244, 30)
/// - 边框: 1.5px 主色 (蓝色)
///
/// ## 颜色配置
/// - `hyperlink_color`: 蓝色 (25, 103, 210)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 5)
/// - `extreme_bg_color`: 极端背景色 (纯白 255)
/// - `code_bg_color`: 代码块背景 (灰 242)
/// - `warn_fg_color`: 深琥珀 (200, 150, 0)
/// - `error_fg_color`: 鲜红 (200, 50, 50)
///
/// ## 窗口样式 (window)
/// - 圆角: 8px (较大)
/// - 阴影: offset=[10,15], blur=25, alpha=50 (清晰柔和)
/// - 填充色: 纯白 (255)
/// - 边框: 1.5px 深灰 (180)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 6px
///
/// ## 面板样式 (panel)
/// - 填充色: 纯白 (255)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[6,10], blur=15, alpha=40
///
/// ## 其他组件
/// - `resize_corner_size`: 12.0
/// - `text_cursor`: 文本光标样式 (2.5px 深蓝, 闪烁 0.4s/0.5s)
/// - `clip_rect_margin`: 4.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: true - 滑块尾部填充
/// - `handle_shape`: Rect(0.8) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.4 - 禁用状态透明度 (清晰可辨)
#[must_use]
pub fn modern_light() -> Style {
    // 颜色定义 (高对比度)
    let background = Color32::from_rgb(255, 255, 255);    // 纯白背景
    let surface = Color32::from_rgb(250, 250, 250);       // 表面色 - 浅灰
    let primary = Color32::from_rgb(25, 103, 210);        // 主色 (深蓝) - 高对比度
    let warning = Color32::from_rgb(200, 150, 0);         // 警告色 (深琥珀)
    let error = Color32::from_rgb(200, 50, 50);           // 错误色 (鲜红)
    let hyperlink = Color32::from_rgb(25, 103, 210);      // 超链接色

    Style {
        visuals: Visuals {
            dark_mode: false,
            override_text_color: None,
            weak_text_alpha: 0.55,
            weak_text_color: None,
            text_alpha_from_coverage: AlphaFromCoverage::LIGHT_MODE_DEFAULT,

            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background,
                    weak_bg_fill: Color32::from_rgb(250, 250, 250),
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(120, 120, 120)),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(30, 30, 30)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(235, 235, 235),
                    weak_bg_fill: Color32::from_rgb(235, 235, 235),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(50, 50, 50)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(66, 133, 244, 25),
                    weak_bg_fill: Color32::from_rgba_premultiplied(66, 133, 244, 25),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(1.5, Color32::BLACK),
                    corner_radius: CornerRadius::same(5),
                    expansion: 2.5,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(66, 133, 244, 35),
                    weak_bg_fill: Color32::from_rgba_premultiplied(66, 133, 244, 35),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::BLACK),
                    corner_radius: CornerRadius::same(4),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: Color32::from_rgb(245, 245, 245),
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(150, 150, 150)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(40, 40, 40)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
            },

            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(25, 103, 210, 30),
                stroke: Stroke::new(1.5, primary),
            },

            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(5),
            extreme_bg_color: Color32::from_gray(255),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(242, 242, 242),
            warn_fg_color: warning,
            error_fg_color: error,

            window_corner_radius: CornerRadius::same(8),
            window_shadow: Shadow {
                offset: [10, 15],
                blur: 25,
                spread: 0,
                color: Color32::from_black_alpha(50),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.5, Color32::from_rgb(180, 180, 180)),
            window_highlight_topmost: true,
            menu_corner_radius: CornerRadius::same(6),
            panel_fill: background,
            popup_shadow: Shadow {
                offset: [6, 10],
                blur: 15,
                spread: 0,
                color: Color32::from_black_alpha(40),
            },
            resize_corner_size: 12.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.5, Color32::from_rgb(0, 80, 150)),
                preview: false,
                blink: true,
                on_duration: 0.4,
                off_duration: 0.5,
            },
            clip_rect_margin: 4.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: true,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.8 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.4,
        },
        ..Default::default()
    }
}
