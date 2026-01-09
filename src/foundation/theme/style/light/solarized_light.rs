//! Solarized Light 亮色主题
//!
//! 特点:
//! - 专为长时间阅读设计，温暖米色调
//! - 适中圆角 (4px)，舒适视觉
//! - 高对比度，清晰易读
//!
//! 颜色来源: Solarized Color Palette
//! 适用场景: 阅读应用、写作软件、代码编辑

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Solarized 亮色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: false - 禁用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.65 - 弱文本透明度
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: LIGHT_MODE_DEFAULT - 亮色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 暖米色 (253, 246, 227)
/// - 弱背景填充: 稍深米色 (248, 242, 223)
/// - 边框: 1.5px 灰褐 (120, 125, 110)
/// - 文字颜色: 1.5px 深青灰 (60, 80, 85)
/// - 圆角: 4px (适中)
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 米色 (238, 232, 213)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 深青灰 (60, 80, 85)
/// - 圆角: 3px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅蓝 (235, 245, 255)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 蓝色 (38, 139, 210)
/// - 文字颜色: 1.5px 近黑 (30, 30, 30)
/// - 圆角: 4px
/// - 扩张: 2.0 (明显悬停效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 蓝色浅 (38, 139, 210, 25)
/// - 弱背景填充: 同上
/// - 边框: 2px 蓝色 (38, 139, 210)
/// - 文字颜色: 2px 纯黑
/// - 圆角: 3px
/// - 扩张: 1.0
///
/// ### open: 展开状态
/// - 背景填充: 表面色 (245, 240, 225)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 灰褐 (130, 135, 120)
/// - 文字颜色: 1px 深青灰 (55, 75, 80)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 黄色 (181, 137, 0, 40)
/// - 边框: 1.5px 黄色 (175, 130, 0)
///
/// ## 颜色配置
/// - `hyperlink_color`: 深蓝 (25, 110, 175)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 3)
/// - `extreme_bg_color`: 极端背景色 (255, 252, 245) - 米白
/// - `code_bg_color`: 代码块背景 (245, 240, 225)
/// - `warn_fg_color`: 深橙黄 (160, 120, 0)
/// - `error_fg_color`: 鲜红 (200, 45, 45)
///
/// ## 窗口样式 (window)
/// - 圆角: 6px
/// - 阴影: offset=[6,10], blur=15, alpha=35 (清晰柔和)
/// - 填充色: 暖米色
/// - 边框: 1.5px 灰褐 (140, 145, 130)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 4px
///
/// ## 面板样式 (panel)
/// - 填充色: 表面色 (245, 240, 225)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[4,8], blur=10, alpha=30
///
/// ## 其他组件
/// - `resize_corner_size`: 10.0
/// - `text_cursor`: 文本光标样式 (2.5px 深蓝, 闪烁 0.5s/0.5s)
/// - `clip_rect_margin`: 3.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: true - 滑块尾部填充
/// - `handle_shape`: Rect(0.7) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.5 - 禁用状态透明度
#[must_use]
pub fn solarized_light() -> Style {
    // Solarized Light 颜色定义 (高对比度)
    let background = Color32::from_rgb(253, 246, 227);   // Base3 - 暖米色
    let surface = Color32::from_rgb(245, 240, 225);      // Surface - 表面色
    let primary = Color32::from_rgb(25, 110, 175);       // 深蓝 - 高对比度
    let warning = Color32::from_rgb(160, 120, 0);        // 深橙黄
    let error = Color32::from_rgb(200, 45, 45);          // 鲜红
    let hyperlink = Color32::from_rgb(25, 110, 175);     // 深蓝

    Style {
        visuals: Visuals {
            dark_mode: false,
            override_text_color: None,
            weak_text_alpha: 0.65,
            weak_text_color: None,
            text_alpha_from_coverage: AlphaFromCoverage::LIGHT_MODE_DEFAULT,

            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background,
                    weak_bg_fill: Color32::from_rgb(248, 242, 223),
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(120, 125, 110)),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(60, 80, 85)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(238, 232, 213),
                    weak_bg_fill: Color32::from_rgb(238, 232, 213),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(60, 80, 85)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(38, 139, 210, 20),
                    weak_bg_fill: Color32::from_rgba_premultiplied(38, 139, 210, 20),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(1.5, Color32::BLACK),
                    corner_radius: CornerRadius::same(4),
                    expansion: 2.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(38, 139, 210, 30),
                    weak_bg_fill: Color32::from_rgba_premultiplied(38, 139, 210, 30),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::BLACK),
                    corner_radius: CornerRadius::same(3),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(130, 135, 120)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(55, 75, 80)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
            },

            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(175, 130, 0, 40),
                stroke: Stroke::new(1.5, Color32::from_rgb(175, 130, 0)),
            },

            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(3),
            extreme_bg_color: Color32::from_rgb(255, 252, 245),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(245, 240, 225),
            warn_fg_color: warning,
            error_fg_color: error,

            window_corner_radius: CornerRadius::same(6),
            window_shadow: Shadow {
                offset: [6, 10],
                blur: 15,
                spread: 0,
                color: Color32::from_black_alpha(35),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.5, Color32::from_rgb(140, 145, 130)),
            window_highlight_topmost: true,
            menu_corner_radius: CornerRadius::same(4),
            panel_fill: surface,
            popup_shadow: Shadow {
                offset: [4, 8],
                blur: 10,
                spread: 0,
                color: Color32::from_black_alpha(30),
            },
            resize_corner_size: 10.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.5, primary),
                preview: false,
                blink: true,
                on_duration: 0.5,
                off_duration: 0.5,
            },
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: true,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.7 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.5,
        },
        ..Default::default()
    }
}
