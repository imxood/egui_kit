//! One Dark 暗色主题
//!
//! 特点:
//! - VS Code 默认暗色主题风格
//! - 暖灰背景，不那么冷
//! - 圆角适中 (6px)，标准阴影
//!
//! 颜色来源: Atom One Dark
//! 适用场景: 开发者工具、代码编辑器

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// One Dark 暗色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: true - 启用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.6 - 弱文本透明度
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: DARK_MODE_DEFAULT - 暗色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 暖灰 (40, 44, 52) - Editor BG
/// - 弱背景填充: 同背景色
/// - 边框: 1px 暖灰 (60, 64, 74)
/// - 文字颜色: 1px 灰 (146, 152, 165)
/// - 圆角: 6px (适中)
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 暖灰 (60, 64, 74)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 灰 (171, 178, 191)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅暖灰 (70, 74, 84)
/// - 弱背景填充: 同上
/// - 边框: 1px 主色 (Blue)
/// - 文字颜色: 1.5px 亮灰 (220, 223, 230)
/// - 圆角: 5px
/// - 扩张: 2.0 (明显的悬停效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 暖灰 (60, 64, 74)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 主色 (Blue)
/// - 文字颜色: 2px 亮灰 (220, 223, 230)
/// - 圆角: 4px
/// - 扩张: 1.0
///
/// ### open: 展开状态
/// - 背景填充: 表面色 (48, 52, 60) - Sidebar BG
/// - 弱背景填充: 同上
/// - 边框: 1px 暖灰 (70, 74, 84)
/// - 文字颜色: 1px 灰 (171, 178, 191)
/// - 圆角: 6px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 暖灰 (62, 68, 80)
/// - 边框: 1px 主色 (Blue)
///
/// ## 颜色配置
/// - `hyperlink_color`: Blue (97, 175, 239)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 8)
/// - `extreme_bg_color`: 极端背景色 (30, 33, 39) - 更深的暖灰
/// - `code_bg_color`: 代码块背景 (45, 49, 56)
/// - `warn_fg_color`: Yellow (229, 192, 123)
/// - `error_fg_color`: Red (224, 108, 117)
///
/// ## 窗口样式 (window)
/// - 圆角: 6px (适中)
/// - 阴影: offset=[10,15], blur=20, alpha=90 (标准阴影)
/// - 填充色: 背景色
/// - 边框: 1px 暖灰
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 6px
///
/// ## 面板样式 (panel)
/// - 填充色: 表面色 (48, 52, 60)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[6,10], blur=12, alpha=80
///
/// ## 其他组件
/// - `resize_corner_size`: 12.0
/// - `text_cursor`: 文本光标样式 (2px 主色, 闪烁 0.5s/0.5s)
/// - `clip_rect_margin`: 4.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: false - 滑块尾部不填充
/// - `handle_shape`: Rect(0.75) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.5 - 禁用状态透明度
#[must_use]
pub fn one_dark() -> Style {
    // One Dark 颜色定义
    let background = Color32::from_rgb(40, 44, 52);      // Editor BG - 暖灰
    let surface = Color32::from_rgb(48, 52, 60);         // Sidebar BG - 稍浅
    let primary = Color32::from_rgb(97, 175, 239);       // Blue - 蓝色
    let warning = Color32::from_rgb(229, 192, 123);      // Yellow - 橙黄
    let error = Color32::from_rgb(224, 108, 117);        // Red - 红色
    let hyperlink = Color32::from_rgb(97, 175, 239);     // Blue - 蓝色

    Style {
        visuals: Visuals {
            // === 基础模式设置 ===
            dark_mode: true, // 启用暗色模式
            override_text_color: None,
            weak_text_alpha: 0.6,
            weak_text_color: None,
            text_alpha_from_coverage: AlphaFromCoverage::DARK_MODE_DEFAULT,

            // === 组件状态视觉 ===
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background,
                    weak_bg_fill: background,
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(60, 64, 74)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(146, 152, 165)),
                    corner_radius: CornerRadius::same(6),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(60, 64, 74),
                    weak_bg_fill: Color32::from_rgb(60, 64, 74),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(171, 178, 191)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgb(70, 74, 84),
                    weak_bg_fill: Color32::from_rgb(70, 74, 84),
                    bg_stroke: Stroke::new(1.0, primary),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(220, 223, 230)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 2.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgb(60, 64, 74),
                    weak_bg_fill: Color32::from_rgb(60, 64, 74),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(2.0, Color32::from_rgb(220, 223, 230)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(70, 74, 84)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(171, 178, 191)),
                    corner_radius: CornerRadius::same(6),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgb(62, 68, 80),
                stroke: Stroke::new(1.0, primary),
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(8),
            extreme_bg_color: Color32::from_rgb(30, 33, 39),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(45, 49, 56),
            warn_fg_color: warning,
            error_fg_color: error,

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(6),
            window_shadow: Shadow {
                offset: [10, 15],
                blur: 20,
                spread: 0,
                color: Color32::from_black_alpha(90),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.0, Color32::from_rgb(60, 64, 74)),
            window_highlight_topmost: true,

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(6),

            // === 面板样式 ===
            panel_fill: surface,

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [6, 10],
                blur: 12,
                spread: 0,
                color: Color32::from_black_alpha(80),
            },

            // === 其他组件样式 ===
            resize_corner_size: 12.0,
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
            handle_shape: HandleShape::Rect { aspect_ratio: 0.75 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.5,
        },
        ..Default::default()
    }
}
