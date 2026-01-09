//! Tokyo Night 暗色主题
//!
//! 特点:
//! - 深蓝渐变色调，日式意境风格
//! - 宁静深邃，梦幻般的感觉
//! - 圆角小 (4px)，阴影梦幻
//!
//! 颜色来源: Tokyo Night Color Palette
//! 适用场景: 夜间使用、阅读应用

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Tokyo Night 暗色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: true - 启用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.65 - 弱文本透明度
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: DARK_MODE_DEFAULT - 暗色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 深蓝黑 (26, 27, 38) - Night
/// - 弱背景填充: 同背景色
/// - 边框: 1px 深蓝灰 (56, 61, 87)
/// - 文字颜色: 1px 灰 (135, 140, 167)
/// - 圆角: 4px (较小)
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 深蓝灰 (56, 61, 87)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 灰 (169, 177, 214)
/// - 圆角: 3px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅深蓝灰 (66, 73, 103)
/// - 弱背景填充: 同上
/// - 边框: 1px 主色 (Blue)
/// - 文字颜色: 1.5px 亮灰 (200, 208, 245)
/// - 圆角: 4px
/// - 扩张: 2.0 (明显的悬停效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 深蓝灰 (56, 61, 87)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 主色 (Blue)
/// - 文字颜色: 2px 纯白
/// - 圆角: 3px
/// - 扩张: 1.0
///
/// ### open: 展开状态
/// - 背景填充: 表面色 (36, 40, 59) - Deep Blue
/// - 弱背景填充: 同上
/// - 边框: 1px 深蓝灰 (66, 73, 103)
/// - 文字颜色: 1px 灰 (169, 177, 214)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 深蓝灰 (56, 61, 87)
/// - 边框: 1px 主色 (Blue)
///
/// ## 颜色配置
/// - `hyperlink_color`: Sky (125, 207, 255)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 6)
/// - `extreme_bg_color`: 极端背景色 (15, 17, 25) - 更深的蓝黑
/// - `code_bg_color`: 代码块背景 (30, 32, 48)
/// - `warn_fg_color`: Yellow (224, 175, 104)
/// - `error_fg_color`: Red (247, 118, 142) - 粉红
///
/// ## 窗口样式 (window)
/// - 圆角: 4px
/// - 阴影: offset=[10,15], blur=25, alpha=90 (梦幻般柔和)
/// - 填充色: 背景色
/// - 边框: 1px 深蓝灰
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 4px
///
/// ## 面板样式 (panel)
/// - 填充色: 表面色 (36, 40, 59)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[6,10], blur=15, alpha=80
///
/// ## 其他组件
/// - `resize_corner_size`: 10.0
/// - `text_cursor`: 文本光标样式 (2px 主色, 闪烁 0.5s/0.5s)
/// - `clip_rect_margin`: 4.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: false - 滑块尾部不填充
/// - `handle_shape`: Rect(0.7) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.55 - 禁用状态透明度
#[must_use]
pub fn tokyo_night() -> Style {
    // Tokyo Night 颜色定义
    let background = Color32::from_rgb(26, 27, 38);      // Night - 深蓝黑
    let surface = Color32::from_rgb(36, 40, 59);         // Deep Blue - 深蓝
    let primary = Color32::from_rgb(122, 162, 247);      // Blue - 蓝色
    let warning = Color32::from_rgb(224, 175, 104);      // Yellow - 橙黄
    let error = Color32::from_rgb(247, 118, 142);        // Red - 粉红
    let hyperlink = Color32::from_rgb(125, 207, 255);    // Sky - 天蓝

    Style {
        visuals: Visuals {
            // === 基础模式设置 ===
            dark_mode: true, // 启用暗色模式
            override_text_color: None,
            weak_text_alpha: 0.65,
            weak_text_color: None,
            text_alpha_from_coverage: AlphaFromCoverage::DARK_MODE_DEFAULT,

            // === 组件状态视觉 ===
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background,
                    weak_bg_fill: background,
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(56, 61, 87)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(135, 140, 167)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(56, 61, 87),
                    weak_bg_fill: Color32::from_rgb(56, 61, 87),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(169, 177, 214)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgb(66, 73, 103),
                    weak_bg_fill: Color32::from_rgb(66, 73, 103),
                    bg_stroke: Stroke::new(1.0, primary),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(200, 208, 245)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 2.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgb(56, 61, 87),
                    weak_bg_fill: Color32::from_rgb(56, 61, 87),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(2.0, Color32::WHITE),
                    corner_radius: CornerRadius::same(3),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(66, 73, 103)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(169, 177, 214)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgb(56, 61, 87),
                stroke: Stroke::new(1.0, primary),
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(6),
            extreme_bg_color: Color32::from_rgb(15, 17, 25),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(30, 32, 48),
            warn_fg_color: warning,
            error_fg_color: error,

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(4),
            window_shadow: Shadow {
                offset: [10, 15],
                blur: 25,
                spread: 0,
                color: Color32::from_black_alpha(90),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.0, Color32::from_rgb(56, 61, 87)),
            window_highlight_topmost: true,

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(4),

            // === 面板样式 ===
            panel_fill: surface,

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [6, 10],
                blur: 15,
                spread: 0,
                color: Color32::from_black_alpha(80),
            },

            // === 其他组件样式 ===
            resize_corner_size: 10.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, primary),
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
            slider_trailing_fill: false,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.7 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.55,
        },
        ..Default::default()
    }
}
