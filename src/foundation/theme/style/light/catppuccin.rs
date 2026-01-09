//! Catppuccin 亮色主题
//!
//! 特点:
//! - 柔和的粉彩色调，优雅舒适
//! - 圆角适中 (6px)，柔和阴影
//! - 高对比度，清晰易读
//!
//! 颜色来源: Catppuccin Latte
//! 适用场景: 创意应用、现代UI、设计工具

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Catppuccin 亮色主题
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
/// - 背景填充色: 浅灰蓝 (239, 241, 245)
/// - 弱背景填充: 纯白 (252, 252, 252)
/// - 边框: 1.5px 灰 (160, 165, 180)
/// - 文字颜色: 1.5px 近黑 (40, 42, 55)
/// - 圆角: 6px (适中)
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 浅灰 (220, 224, 232)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 深灰 (49, 50, 68)
/// - 圆角: 5px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅紫粉 (243, 232, 255)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 主色 (蓝色)
/// - 文字颜色: 1.5px 纯黑
/// - 圆角: 6px
/// - 扩张: 2.5 (明显的悬停效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 主色浅 (30, 102, 245, 20)
/// - 弱背景填充: 同上
/// - 边框: 2px 主色 (蓝色)
/// - 文字颜色: 2px 纯黑
/// - 圆角: 5px
/// - 扩张: 1.0
///
/// ### open: 展开状态
/// - 背景填充: 表面色 (245, 245, 250)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 灰 (170, 175, 190)
/// - 文字颜色: 1px 深灰 (55, 57, 75)
/// - 圆角: 6px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 主色浅 (30, 102, 245, 25)
/// - 边框: 1.5px 主色 (蓝色)
///
/// ## 颜色配置
/// - `hyperlink_color`: 深蓝 (20, 85, 200)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 5)
/// - `extreme_bg_color`: 极端背景色 (纯白 255)
/// - `code_bg_color`: 代码块背景 (235, 238, 245)
/// - `warn_fg_color`: 深橙黄 (175, 110, 20)
/// - `error_fg_color`: 鲜红 (185, 20, 50)
///
/// ## 窗口样式 (window)
/// - 圆角: 8px (较大)
/// - 阴影: offset=[10,15], blur=25, alpha=45 (清晰柔和)
/// - 填充色: 纯白 (255)
/// - 边框: 1.5px 灰 (180, 185, 200)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 6px
///
/// ## 面板样式 (panel)
/// - 填充色: 浅灰蓝 (245, 247, 252)
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
/// - `disabled_alpha`: 0.45 - 禁用状态透明度
#[must_use]
pub fn catppuccin() -> Style {
    // Catppuccin Latte 颜色定义 (高对比度)
    let background = Color32::from_rgb(255, 255, 255);    // 纯白
    let surface = Color32::from_rgb(245, 247, 252);       // 浅灰蓝
    let primary = Color32::from_rgb(20, 85, 200);         // 深蓝 - 高对比度
    let warning = Color32::from_rgb(175, 110, 20);        // 深橙黄
    let error = Color32::from_rgb(185, 20, 50);           // 鲜红
    let hyperlink = Color32::from_rgb(20, 85, 200);       // 深蓝

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
                    weak_bg_fill: Color32::from_rgb(252, 252, 252),
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(160, 165, 180)),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(40, 42, 55)),
                    corner_radius: CornerRadius::same(6),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(220, 224, 232),
                    weak_bg_fill: Color32::from_rgb(220, 224, 232),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(49, 50, 68)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(30, 102, 245, 18),
                    weak_bg_fill: Color32::from_rgba_premultiplied(30, 102, 245, 18),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(1.5, Color32::BLACK),
                    corner_radius: CornerRadius::same(6),
                    expansion: 2.5,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(30, 102, 245, 25),
                    weak_bg_fill: Color32::from_rgba_premultiplied(30, 102, 245, 25),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::BLACK),
                    corner_radius: CornerRadius::same(5),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(170, 175, 190)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(55, 57, 75)),
                    corner_radius: CornerRadius::same(6),
                    expansion: 0.0,
                },
            },

            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(20, 85, 200, 25),
                stroke: Stroke::new(1.5, primary),
            },

            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(5),
            extreme_bg_color: Color32::from_gray(255),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(235, 238, 245),
            warn_fg_color: warning,
            error_fg_color: error,

            window_corner_radius: CornerRadius::same(8),
            window_shadow: Shadow {
                offset: [10, 15],
                blur: 25,
                spread: 0,
                color: Color32::from_black_alpha(45),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.5, Color32::from_rgb(180, 185, 200)),
            window_highlight_topmost: true,
            menu_corner_radius: CornerRadius::same(6),
            panel_fill: surface,
            popup_shadow: Shadow {
                offset: [6, 10],
                blur: 15,
                spread: 0,
                color: Color32::from_black_alpha(40),
            },
            resize_corner_size: 12.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.5, primary),
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
            disabled_alpha: 0.45,
        },
        ..Default::default()
    }
}
