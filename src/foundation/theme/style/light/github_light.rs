//! GitHub Light 亮色主题
//!
//! 特点:
//! - GitHub 官网视觉风格，干净简洁
//! - 圆角适中 (5px)，清晰边框
//! - 高对比度，清晰易读
//!
//! 颜色来源: GitHub Light Color Palette
//! 适用场景: 开发者工具、文档应用

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// GitHub 亮色主题
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
/// - 背景填充色: 纯白 (255, 255, 255)
/// - 弱背景填充: 浅灰 (248, 248, 248)
/// - 边框: 1.5px 深灰 (100, 100, 100)
/// - 文字颜色: 1.5px 近黑 (25, 25, 25)
/// - 圆角: 5px (适中)
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 浅灰 (240, 240, 240)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 深灰 (50, 50, 50)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅蓝 (240, 248, 255)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 主色 (蓝色)
/// - 文字颜色: 1.5px 纯黑
/// - 圆角: 5px
/// - 扩张: 2.0 (明显的悬停效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 主色浅 (9, 105, 218, 20)
/// - 弱背景填充: 同上
/// - 边框: 2px 主色 (蓝色)
/// - 文字颜色: 2px 纯黑
/// - 圆角: 4px
/// - 扩张: 1.0
///
/// ### open: 展开状态
/// - 背景填充: 表面色 (250, 250, 250)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 深灰 (130, 130, 130)
/// - 文字颜色: 1px 深灰 (40, 40, 40)
/// - 圆角: 5px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 主色浅 (9, 105, 218, 25)
/// - 边框: 1.5px 主色 (蓝色)
///
/// ## 颜色配置
/// - `hyperlink_color`: 蓝色 (5, 80, 175)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 3)
/// - `extreme_bg_color`: 极端背景色 (纯白 255)
/// - `code_bg_color`: 代码块背景 (247, 247, 247)
/// - `warn_fg_color`: 深橙黄 (175, 115, 0)
/// - `error_fg_color`: 鲜红 (190, 30, 45)
///
/// ## 窗口样式 (window)
/// - 圆角: 6px
/// - 阴影: offset=[8,12], blur=20, alpha=45 (清晰柔和)
/// - 填充色: 纯白
/// - 边框: 1.5px 深灰 (160)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 5px
///
/// ## 面板样式 (panel)
/// - 填充色: 表面色 (248, 248, 248)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[5,8], blur=12, alpha=35
///
/// ## 其他组件
/// - `resize_corner_size`: 10.0
/// - `text_cursor`: 文本光标样式 (2.5px 主色, 闪烁 0.4s/0.5s)
/// - `clip_rect_margin`: 3.0
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: true - 滑块尾部填充
/// - `handle_shape`: Rect(0.75) - 滑块句柄长方形
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.4 - 禁用状态透明度
#[must_use]
pub fn github_light() -> Style {
    // GitHub Light 颜色定义 (高对比度)
    let background = Color32::from_rgb(255, 255, 255);   // 纯白
    let surface = Color32::from_rgb(248, 248, 248);      // 浅灰
    let primary = Color32::from_rgb(5, 80, 175);         // 深蓝 - 高对比度
    let warning = Color32::from_rgb(175, 115, 0);        // 深橙黄
    let error = Color32::from_rgb(190, 30, 45);          // 鲜红
    let hyperlink = Color32::from_rgb(5, 80, 175);       // 深蓝

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
                    weak_bg_fill: Color32::from_rgb(248, 248, 248),
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(100, 100, 100)),
                    fg_stroke: Stroke::new(1.5, Color32::from_rgb(25, 25, 25)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(240, 240, 240),
                    weak_bg_fill: Color32::from_rgb(240, 240, 240),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(50, 50, 50)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(9, 105, 218, 20),
                    weak_bg_fill: Color32::from_rgba_premultiplied(9, 105, 218, 20),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(1.5, Color32::BLACK),
                    corner_radius: CornerRadius::same(5),
                    expansion: 2.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(9, 105, 218, 30),
                    weak_bg_fill: Color32::from_rgba_premultiplied(9, 105, 218, 30),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::BLACK),
                    corner_radius: CornerRadius::same(4),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.5, Color32::from_rgb(130, 130, 130)),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(40, 40, 40)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 0.0,
                },
            },

            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(5, 80, 175, 25),
                stroke: Stroke::new(1.5, primary),
            },

            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(3),
            extreme_bg_color: Color32::from_gray(255),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(247, 247, 247),
            warn_fg_color: warning,
            error_fg_color: error,

            window_corner_radius: CornerRadius::same(6),
            window_shadow: Shadow {
                offset: [8, 12],
                blur: 20,
                spread: 0,
                color: Color32::from_black_alpha(45),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.5, Color32::from_rgb(160, 160, 160)),
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
            disabled_alpha: 0.4,
        },
        ..Default::default()
    }
}
