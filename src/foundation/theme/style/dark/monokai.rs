//! Monokai Pro 暗色主题
//!
//! 特点:
//! - 专业代码编辑器风格, 温暖色调
//! - 高对比度, 清晰易读
//! - 圆角适中 (5px), 柔和视觉效果
//!
//! 颜色来源: Monokai Pro Color Palette
//! 适用场景: 代码编辑器、IDE、设计工具

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Monokai Pro 暗色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: true - 启用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.8 - 弱文本透明度较高
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: DARK_MODE_DEFAULT - 暗色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 深褐灰 (40, 35, 35)
/// - 弱背景填充: 同背景色
/// - 边框: 1px 暖灰 (90, 85, 85)
/// - 文字颜色: 1px 米白 (235, 230, 220)
/// - 圆角: 5px
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 暖褐 (55, 50, 50)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 米白 (245, 240, 230)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 珊瑚橙 (255, 110, 80, 40)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 珊瑚橙 (255, 110, 80)
/// - 文字颜色: 1.5px 米白 (250, 245, 235)
/// - 圆角: 5px
/// - 扩张: 2.5
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 珊瑚橙 (255, 110, 80, 75)
/// - 弱背景填充: 同上
/// - 边框: 2px 珊瑚橙 (255, 110, 80)
/// - 文字颜色: 2px 米白 (255, 250, 245)
/// - 圆角: 4px
/// - 扩张: 1.5
///
/// ### open: 展开状态
/// - 背景填充: 暖褐 (50, 45, 45)
/// - 弱背景填充: 同上
/// - 边框: 1px 珊瑚橙 (255, 110, 80)
/// - 文字颜色: 1px 米白 (250, 245, 235)
/// - 圆角: 5px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 霓虹紫 (200, 100, 255, 45)
/// - 边框: 1px 霓虹紫 (200, 100, 255)
///
/// ## 颜色配置
/// - `hyperlink_color`: 金橙 (255, 180, 50)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 8)
/// - `extreme_bg_color`: 极端背景色 (深褐 30)
/// - `code_bg_color`: 代码块背景 (50, 45, 45)
/// - `warn_fg_color`: 金黄 (255, 200, 50)
/// - `error_fg_color`: 鲜红 (255, 70, 70)
///
/// ## 窗口样式 (window)
/// - 圆角: 5px
/// - 阴影: offset=[5,10], blur=18, alpha=70
/// - 填充色: 深褐灰
/// - 边框: 1px 暖灰 (80, 75, 75)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 5px
///
/// ## 面板样式 (panel)
/// - 填充色: 暖褐 (45, 40, 40)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[4,10], blur=15, alpha=60
///
/// ## 其他组件
/// - `resize_corner_size`: 11.0
/// - `text_cursor`: 文本光标样式 (2px 金橙, 闪烁 0.5s/0.5s)
/// - `clip_rect_margin`: 4.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: true - 滑块尾部填充
/// - `handle_shape`: Rect(0.85) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.6 - 禁用状态透明度
#[must_use]
pub fn monokai() -> Style {
    // Monokai Pro 颜色定义
    let background = Color32::from_rgb(40, 35, 35);      // 深褐灰
    let surface = Color32::from_rgb(45, 40, 40);        // 暖褐
    let primary = Color32::from_rgb(255, 110, 80);      // Coral - 珊瑚橙
    let secondary = Color32::from_rgb(200, 100, 255);   // Neon Purple - 霓虹紫
    let warning = Color32::from_rgb(255, 200, 50);      // Golden Yellow - 金黄
    let error = Color32::from_rgb(255, 70, 70);         // Bright Red - 鲜红
    let hyperlink = Color32::from_rgb(255, 180, 50);    // Gold Orange - 金橙

    Style {
        visuals: Visuals {
            // === 基础模式设置 ===
            dark_mode: true, // 启用暗色模式
            override_text_color: None,
            weak_text_alpha: 0.8,
            weak_text_color: None,
            text_alpha_from_coverage: AlphaFromCoverage::DARK_MODE_DEFAULT,

            // === 组件状态视觉 ===
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background,
                    weak_bg_fill: background,
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(90, 85, 85)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(235, 230, 220)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(55, 50, 50),
                    weak_bg_fill: Color32::from_rgb(55, 50, 50),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(245, 240, 230)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(255, 110, 80, 40),
                    weak_bg_fill: Color32::from_rgba_premultiplied(255, 110, 80, 40),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(250, 245, 235)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 2.5,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(255, 110, 80, 75),
                    weak_bg_fill: Color32::from_rgba_premultiplied(255, 110, 80, 75),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::from_rgb(255, 250, 245)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 1.5,
                },
                open: WidgetVisuals {
                    bg_fill: Color32::from_rgb(50, 45, 45),
                    weak_bg_fill: Color32::from_rgb(50, 45, 45),
                    bg_stroke: Stroke::new(1.0, primary),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(250, 245, 235)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(200, 100, 255, 45),
                stroke: Stroke::new(1.0, secondary),
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(8),
            extreme_bg_color: Color32::from_rgb(30, 28, 28),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(50, 45, 45),
            warn_fg_color: warning,
            error_fg_color: error,

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(5),
            window_shadow: Shadow {
                offset: [5, 10],
                blur: 18,
                spread: 0,
                color: Color32::from_black_alpha(70),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.0, Color32::from_rgb(80, 75, 75)),
            window_highlight_topmost: true,

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(5),

            // === 面板样式 ===
            panel_fill: surface,

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [4, 10],
                blur: 15,
                spread: 0,
                color: Color32::from_black_alpha(60),
            },

            // === 其他组件样式 ===
            resize_corner_size: 11.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, hyperlink),
                preview: false,
                blink: true,
                on_duration: 0.5,
                off_duration: 0.5,
            },
            clip_rect_margin: 4.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: true,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.85 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.6,
        },
        ..Default::default()
    }
}
