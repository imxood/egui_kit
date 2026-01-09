//! Matrix 暗色主题
//!
//! 特点:
//! - 经典的黑客帝国绿色调
//! - 矩阵风格, 科技感强
//! - 圆角小 (3px), 扁平化设计
//!
//! 颜色来源: Matrix Movie Color Palette
//! 适用场景: 极客工具、代码编辑器、科技演示

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Matrix 暗色主题
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
/// - 背景填充色: 纯黑 (0, 0, 0)
/// - 弱背景填充: 深绿黑 (0, 20, 10)
/// - 边框: 1px 暗绿 (0, 80, 40)
/// - 文字颜色: 1px 灰绿 (160, 200, 170)
/// - 圆角: 3px
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 深绿黑 (5, 25, 15)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 灰绿 (180, 210, 180)
/// - 圆角: 2px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 矩阵绿 (0, 255, 70, 35)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 矩阵绿 (0, 255, 70)
/// - 文字颜色: 1.5px 亮绿 (100, 255, 120)
/// - 圆角: 3px
/// - 扩张: 2.0
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 矩阵绿 (0, 255, 70, 70)
/// - 弱背景填充: 同上
/// - 边框: 2px 矩阵绿 (0, 255, 70)
/// - 文字颜色: 2px 亮绿 (150, 255, 150)
/// - 圆角: 2px
/// - 扩张: 1.0
///
/// ### open: 展开状态
/// - 背景填充: 深绿 (10, 35, 20)
/// - 弱背景填充: 同上
/// - 边框: 1px 矩阵绿 (0, 255, 70)
/// - 文字颜色: 1px 亮绿 (100, 255, 120)
/// - 圆角: 3px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 矩阵绿 (0, 255, 70, 40)
/// - 边框: 1px 矩阵绿 (0, 255, 70)
///
/// ## 颜色配置
/// - `hyperlink_color`: 亮绿 (0, 255, 70)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 4)
/// - `extreme_bg_color`: 极端背景色 (纯黑 0)
/// - `code_bg_color`: 代码块背景 (10, 30, 15)
/// - `warn_fg_color`: 琥珀色 (255, 191, 0)
/// - `error_fg_color`: 红色 (255, 80, 80)
///
/// ## 窗口样式 (window)
/// - 圆角: 3px
/// - 阴影: offset=[4,8], blur=15, alpha=50 (柔和发光)
/// - 填充色: 纯黑
/// - 边框: 1px 暗绿 (0, 80, 40)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 3px
///
/// ## 面板样式 (panel)
/// - 填充色: 深绿黑 (10, 30, 15)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[3,8], blur=12, alpha=45
///
/// ## 其他组件
/// - `resize_corner_size`: 9.0
/// - `text_cursor`: 文本光标样式 (2px 矩阵绿, 闪烁 0.45s/0.45s)
/// - `clip_rect_margin`: 3.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: true - 滑块尾部填充 (绿色效果)
/// - `handle_shape`: Rect(0.75) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.55 - 禁用状态透明度
#[must_use]
pub fn matrix() -> Style {
    // Matrix 颜色定义
    let background = Color32::from_rgb(0, 0, 0);         // 纯黑背景
    let surface = Color32::from_rgb(10, 30, 15);        // 深绿黑表面
    let primary = Color32::from_rgb(0, 255, 70);        // Matrix Green - 矩阵绿
    let warning = Color32::from_rgb(255, 191, 0);       // Amber - 琥珀色
    let error = Color32::from_rgb(255, 80, 80);         // Soft Red - 柔和红
    let hyperlink = Color32::from_rgb(0, 255, 70);      // Matrix Green - 矩阵绿

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
                    weak_bg_fill: Color32::from_rgb(0, 20, 10),
                    bg_stroke: Stroke::new(1.0, Color32::from_rgb(0, 80, 40)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(160, 200, 170)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(5, 25, 15),
                    weak_bg_fill: Color32::from_rgb(5, 25, 15),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(180, 210, 180)),
                    corner_radius: CornerRadius::same(2),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(0, 255, 70, 35),
                    weak_bg_fill: Color32::from_rgba_premultiplied(0, 255, 70, 35),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(100, 255, 120)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 2.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(0, 255, 70, 70),
                    weak_bg_fill: Color32::from_rgba_premultiplied(0, 255, 70, 70),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::from_rgb(150, 255, 150)),
                    corner_radius: CornerRadius::same(2),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: Color32::from_rgb(10, 35, 20),
                    weak_bg_fill: Color32::from_rgb(10, 35, 20),
                    bg_stroke: Stroke::new(1.0, primary),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(100, 255, 120)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(0, 255, 70, 40),
                stroke: Stroke::new(1.0, primary),
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(4),
            extreme_bg_color: Color32::from_gray(0),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(10, 30, 15),
            warn_fg_color: warning,
            error_fg_color: error,

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(3),
            window_shadow: Shadow {
                offset: [4, 8],
                blur: 15,
                spread: 0,
                color: Color32::from_black_alpha(50),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.0, Color32::from_rgb(0, 80, 40)),
            window_highlight_topmost: true,

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(3),

            // === 面板样式 ===
            panel_fill: surface,

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [3, 8],
                blur: 12,
                spread: 0,
                color: Color32::from_black_alpha(45),
            },

            // === 其他组件样式 ===
            resize_corner_size: 9.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, primary),
                preview: false,
                blink: true,
                on_duration: 0.45,
                off_duration: 0.45,
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
            disabled_alpha: 0.55,
        },
        ..Default::default()
    }
}
