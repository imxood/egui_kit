//! Deep Black 暗色主题
//!
//! 特点:
//! - 纯黑背景，纯白文字，超高对比度
//! - 极致清晰，适合视力较弱用户
//! - 圆角小 (3px)，扁平化设计
//!
//! 适用场景: 视力辅助、高对比度需求、代码编辑

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Deep Black 暗色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: true - 启用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色
/// - `weak_text_alpha`: 0.8 - 弱文本透明度较高 (次要文字清晰)
/// - `weak_text_color`: None - 弱文本颜色
/// - `text_alpha_from_coverage`: DARK_MODE_DEFAULT - 暗色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 纯黑 (0, 0, 0)
/// - 弱背景填充: 深灰 (10, 10, 10)
/// - 边框: 1px 灰 (80, 80, 80)
/// - 文字颜色: 1px 纯白 (255, 255, 255)
/// - 圆角: 3px (扁平)
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 深灰 (20, 20, 20)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 亮灰 (220, 220, 220)
/// - 圆角: 2px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 灰 (45, 45, 45)
/// - 弱背景填充: 同上
/// - 边框: 1px 主色 (亮蓝)
/// - 文字颜色: 1.5px 纯白
/// - 圆角: 3px
/// - 扩张: 2.0
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 灰 (60, 60, 60)
/// - 弱背景填充: 同上
/// - 边框: 2px 主色 (亮蓝)
/// - 文字颜色: 2px 纯白
/// - 圆角: 2px
/// - 扩张: 1.0
///
/// ### open: 展开状态
/// - 背景填充: 深灰 (15, 15, 15)
/// - 弱背景填充: 深灰 (15, 15, 15)
/// - 边框: 1px 灰 (60, 60, 60)
/// - 文字颜色: 1px 纯白
/// - 圆角: 3px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 亮蓝 (66, 133, 244, 100)
/// - 边框: 1px 亮蓝
///
/// ## 颜色配置
/// - `hyperlink_color`: 亮蓝 (66, 133, 244)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 3)
/// - `extreme_bg_color`: 极端背景色 (纯黑 0)
/// - `code_bg_color`: 代码块背景 (25, 25, 25)
/// - `warn_fg_color`: 橙黄 (255, 193, 7)
/// - `error_fg_color`: 红色 (244, 67, 54)
///
/// ## 窗口样式 (window)
/// - 圆角: 3px (最小)
/// - 阴影: offset=[4,8], blur=12, alpha=60 (柔和)
/// - 填充色: 纯黑
/// - 边框: 1px 灰 (60, 60, 60)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 3px
///
/// ## 面板样式 (panel)
/// - 填充色: 深灰 (15, 15, 15)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[3,6], blur=8, alpha=50
///
/// ## 其他组件
/// - `resize_corner_size`: 8.0 (较小)
/// - `text_cursor`: 文本光标样式 (2px 亮蓝, 闪烁 0.5s/0.5s)
/// - `clip_rect_margin`: 2.0 (最小边距)
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: false - 滑块尾部不填充
/// - `handle_shape`: Rect(0.7) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.7 - 禁用状态透明度 (保持清晰)
#[must_use]
pub fn deep_black() -> Style {
    // Deep Black 颜色定义
    let background = Color32::from_rgb(0, 0, 0);         // 纯黑背景
    let surface = Color32::from_rgb(15, 15, 15);        // 表面色 (深灰)
    let primary = Color32::from_rgb(66, 133, 244);      // 主色 (亮蓝)
    let warning = Color32::from_rgb(255, 193, 7);       // 橙黄
    let error = Color32::from_rgb(244, 67, 54);         // 红色
    let hyperlink = Color32::from_rgb(66, 133, 244);    // 超链接色

    Style {
        visuals: Visuals {
            // === 基础模式设置 ===
            dark_mode: true, // 启用暗色模式
            override_text_color: None,
            weak_text_alpha: 0.8, // 弱文本保持高可见性
            weak_text_color: None,
            text_alpha_from_coverage: AlphaFromCoverage::DARK_MODE_DEFAULT,

            // === 组件状态视觉 ===
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background, // 纯黑背景
                    weak_bg_fill: Color32::from_rgb(10, 10, 10),
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(80, 80, 80)),
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(255)), // 纯白文字
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(20, 20, 20),
                    weak_bg_fill: Color32::from_rgb(20, 20, 20),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(220)),
                    corner_radius: CornerRadius::same(2),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgb(45, 45, 45),
                    weak_bg_fill: Color32::from_rgb(45, 45, 45),
                    bg_stroke: Stroke::new(1.0, primary),
                    fg_stroke: Stroke::new(1.5, Color32::from_gray(255)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 2.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgb(60, 60, 60),
                    weak_bg_fill: Color32::from_rgb(60, 60, 60),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::from_gray(255)),
                    corner_radius: CornerRadius::same(2),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(60, 60, 60)),
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(255)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(66, 133, 244, 100),
                stroke: Stroke::new(1.0, primary),
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(3),
            extreme_bg_color: Color32::from_gray(0), // 纯黑
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(25, 25, 25),
            warn_fg_color: warning,
            error_fg_color: error,

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(3),
            window_shadow: Shadow {
                offset: [4, 8],
                blur: 12,
                spread: 0,
                color: Color32::from_black_alpha(60),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.0, Color32::from_rgb(60, 60, 60)),
            window_highlight_topmost: true,

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(3),

            // === 面板样式 ===
            panel_fill: surface,

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [3, 6],
                blur: 8,
                spread: 0,
                color: Color32::from_black_alpha(50),
            },

            // === 其他组件样式 ===
            resize_corner_size: 8.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, primary),
                preview: false,
                blink: true,
                on_duration: 0.5,
                off_duration: 0.5,
            },
            clip_rect_margin: 2.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: false,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.7 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.7, // 禁用状态保持较高可见性
        },
        ..Default::default()
    }
}
