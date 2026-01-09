//! Cyberpunk 暗色主题
//!
//! 特点:
//! - 赛博朋克风格，霓虹粉/蓝对比
//! - 高对比度设计，未来科技感
//! - 圆角小 (4px)，霓虹发光效果
//!
//! 颜色来源: Cyberpunk 2077 Color Palette
//! 适用场景: 游戏UI、科技应用、未来风格应用

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Cyberpunk 暗色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: true - 启用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.7 - 弱文本透明度
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: DARK_MODE_DEFAULT - 暗色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 深蓝黑 (10, 10, 18)
/// - 弱背景填充: 同背景色
/// - 边框: 1px 灰紫 (60, 55, 75)
/// - 文字颜色: 1px 灰 (180, 180, 190)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 蓝灰 (30, 30, 50)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 灰 (200, 200, 210)
/// - 圆角: 3px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 霓虹粉 (255, 0, 128, 40) - 微妙发光
/// - 弱背景填充: 同上
/// - 边框: 1.5px 霓虹粉 (255, 0, 128)
/// - 文字颜色: 1.5px 亮青 (0, 255, 255)
/// - 圆角: 4px
/// - 扩张: 2.5 (明显的悬停放大效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 霓虹粉 (255, 0, 128, 80)
/// - 弱背景填充: 同上
/// - 边框: 2px 霓虹粉 (255, 0, 128)
/// - 文字颜色: 2px 亮青 (0, 255, 255)
/// - 圆角: 3px
/// - 扩张: 1.5
///
/// ### open: 展开状态
/// - 背景填充: 深紫 (20, 18, 35)
/// - 弱背景填充: 同上
/// - 边框: 1px 霓虹蓝 (0, 191, 255)
/// - 文字颜色: 1px 亮青 (0, 255, 255)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 霓虹粉 (255, 0, 128, 50)
/// - 边框: 1px 霓虹粉 (255, 0, 128)
///
/// ## 颜色配置
/// - `hyperlink_color`: 霓虹蓝 (0, 191, 255)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 5)
/// - `extreme_bg_color`: 极端背景色 (5, 5, 10)
/// - `code_bg_color`: 代码块背景 (25, 25, 40)
/// - `warn_fg_color`: 霓虹黄 (255, 255, 0)
/// - `error_fg_color`: 霓虹红 (255, 50, 50)
///
/// ## 窗口样式 (window)
/// - 圆角: 4px
/// - 阴影: offset=[8,12], blur=20, alpha=80 (霓虹发光感)
/// - 填充色: 深蓝黑
/// - 边框: 1px 霓虹粉 (255, 0, 128)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 4px
///
/// ## 面板样式 (panel)
/// - 填充色: 深紫 (20, 18, 35)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[5,10], blur=15, alpha=70
///
/// ## 其他组件
/// - `resize_corner_size`: 10.0
/// - `text_cursor`: 文本光标样式 (2px 霓虹青, 闪烁 0.4s/0.4s 快速)
/// - `clip_rect_margin`: 3.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: true - 滑块尾部填充 (霓虹效果)
/// - `handle_shape`: Rect(0.8) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.5 - 禁用状态透明度
#[must_use]
pub fn cyberpunk() -> Style {
    // Cyberpunk 颜色定义
    let background = Color32::from_rgb(10, 10, 18);     // 深蓝黑
    let surface = Color32::from_rgb(20, 18, 35);        // 深紫
    let primary = Color32::from_rgb(0, 191, 255);       // Neon Blue - 霓虹蓝
    let accent = Color32::from_rgb(255, 0, 128);        // Neon Pink - 霓虹粉
    let warning = Color32::from_rgb(255, 255, 0);       // Neon Yellow - 霓虹黄
    let error = Color32::from_rgb(255, 50, 50);         // Neon Red - 霓虹红
    let hyperlink = Color32::from_rgb(0, 191, 255);     // Neon Blue - 霓虹蓝

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
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(60, 55, 75)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(180, 180, 190)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(30, 30, 50),
                    weak_bg_fill: Color32::from_rgb(30, 30, 50),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(200, 200, 210)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(255, 0, 128, 40),
                    weak_bg_fill: Color32::from_rgba_premultiplied(255, 0, 128, 40),
                    bg_stroke: Stroke::new(1.5, accent), // 霓虹粉边框
                    fg_stroke: Stroke::new(1.5, primary), // 霓虹青文字
                    corner_radius: CornerRadius::same(4),
                    expansion: 2.5, // 明显的悬停效果
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(255, 0, 128, 80),
                    weak_bg_fill: Color32::from_rgba_premultiplied(255, 0, 128, 80),
                    bg_stroke: Stroke::new(2.0, accent),
                    fg_stroke: Stroke::new(2.0, primary),
                    corner_radius: CornerRadius::same(3),
                    expansion: 1.5,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.0, primary),
                    fg_stroke: Stroke::new(1.0, primary),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(255, 0, 128, 50),
                stroke: Stroke::new(1.0, accent),
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(5),
            extreme_bg_color: Color32::from_rgb(5, 5, 10),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(25, 25, 40),
            warn_fg_color: warning,
            error_fg_color: error,

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(4),
            window_shadow: Shadow {
                offset: [8, 12],
                blur: 20,
                spread: 0,
                color: Color32::from_black_alpha(80),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.0, accent), // 霓虹粉边框
            window_highlight_topmost: true,

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(4),

            // === 面板样式 ===
            panel_fill: surface,

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [5, 10],
                blur: 15,
                spread: 0,
                color: Color32::from_black_alpha(70),
            },

            // === 其他组件样式 ===
            resize_corner_size: 10.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, primary), // 霓虹青光标
                preview: false,
                blink: true,
                on_duration: 0.4, // 快速闪烁
                off_duration: 0.4,
            },
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: true, // 霓虹效果
            handle_shape: HandleShape::Rect { aspect_ratio: 0.8 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.5,
        },
        ..Default::default()
    }
}
