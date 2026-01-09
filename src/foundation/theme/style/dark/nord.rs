//! Nord 暗色主题
//!
//! 特点:
//! - 北欧极简风格，冷色调蓝色系
//! - 圆角更小 (4px)，更扁平化
//! - 阴影更淡，柔和护眼
//!
//! 颜色来源: Nord Color Palette
//! 适用场景: 长时间阅读、代码编辑器

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// Nord 暗色主题
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
/// - 背景填充色: 深蓝灰 (46, 52, 64)
/// - 弱背景填充: 同背景色
/// - 边框: 1px 灰 (70)
/// - 文字颜色: 1px 灰 (150)
/// - 圆角: 4px (更扁平)
/// - 扩张: 0.0
///
/// ### inactive: 非激活状态
/// - 背景填充: 蓝灰 (67, 74, 90)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 亮灰 (180)
/// - 圆角: 3px (更小)
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅蓝灰 (76, 86, 106)
/// - 弱背景填充: 同上
/// - 边框: 1px 主色 (Frost 蓝)
/// - 文字颜色: 1.5px 亮灰 (220)
/// - 圆角: 4px
/// - 扩张: 1.5 (较小的悬停效果)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 蓝灰 (67, 74, 90)
/// - 弱背景填充: 同上
/// - 边框: 2px 主色 (更粗的边框)
/// - 文字颜色: 2px 纯白
/// - 圆角: 3px
/// - 扩张: 0.5 (轻微收缩)
///
/// ### open: 展开状态
/// - 背景填充: 表面色 (59, 66, 82)
/// - 弱背景填充: 同上
/// - 边框: 1px 灰 (80)
/// - 文字颜色: 1px 灰 (200)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 蓝灰 (67, 74, 90)
/// - 边框: 1px 主色 (Frost 蓝)
///
/// ## 颜色配置
/// - `hyperlink_color`: Frost 青色 (136, 192, 208)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 5)
/// - `extreme_bg_color`: 极端背景色 (30, 34, 42)
/// - `code_bg_color`: 代码块背景 (40, 46, 58)
/// - `warn_fg_color`: Aurora 黄色 (235, 203, 139)
/// - `error_fg_color`: Aurora 红色 (191, 97, 106)
///
/// ## 窗口样式 (window)
/// - 圆角: 4px (更扁平)
/// - 阴影: offset=[8,12], blur=16, alpha=70 (更淡)
/// - 填充色: 背景色
/// - 边框: 1px 灰 (70)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 4px
///
/// ## 面板样式 (panel)
/// - 填充色: 表面色 (59, 66, 82)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[4,8], blur=10, alpha=60 (更淡)
///
/// ## 其他组件
/// - `resize_corner_size`: 10.0 - 调整角较小
/// - `text_cursor`: 文本光标样式 (2px 主色, 闪烁 0.6s/0.4s)
/// - `clip_rect_margin`: 3.0 - 剪裁边距较小
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: false - 滑块尾部不填充
/// - `handle_shape`: Rect(0.6) - 滑块句柄细长形，长寬比 0.6
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.6 - 禁用状态透明度较高
#[must_use]
pub fn nord() -> Style {
    // Nord 颜色定义
    let background = Color32::from_rgb(46, 52, 64);       // Polar Night - 深蓝灰
    let surface = Color32::from_rgb(59, 66, 82);         // Polar Night - 稍浅
    let primary = Color32::from_rgb(94, 129, 172);       // Frost - 蓝色
    let warning = Color32::from_rgb(235, 203, 139);      // Aurora - 黄色
    let error = Color32::from_rgb(191, 97, 106);         // Aurora - 红色
    let hyperlink = Color32::from_rgb(136, 192, 208);    // Frost - 青色

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
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(70)),
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(150)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgb(67, 74, 90),
                    weak_bg_fill: Color32::from_rgb(67, 74, 90),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(180)),
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_rgb(76, 86, 106),
                    weak_bg_fill: Color32::from_rgb(76, 86, 106),
                    bg_stroke: Stroke::new(1.0, primary),
                    fg_stroke: Stroke::new(1.5, Color32::from_gray(220)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 1.5,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_rgb(67, 74, 90),
                    weak_bg_fill: Color32::from_rgb(67, 74, 90),
                    bg_stroke: Stroke::new(2.0, primary),
                    fg_stroke: Stroke::new(2.0, Color32::WHITE),
                    corner_radius: CornerRadius::same(3),
                    expansion: 0.5,
                },
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: surface,
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(80)),
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(200)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgb(67, 74, 90),
                stroke: Stroke::new(1.0, primary),
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink,
            faint_bg_color: Color32::from_additive_luminance(5),
            extreme_bg_color: Color32::from_rgb(30, 34, 42),
            text_edit_bg_color: None,
            code_bg_color: Color32::from_rgb(40, 46, 58),
            warn_fg_color: warning,
            error_fg_color: error,

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(4),
            window_shadow: Shadow {
                offset: [8, 12],
                blur: 16,
                spread: 0,
                color: Color32::from_black_alpha(70),
            },
            window_fill: background,
            window_stroke: Stroke::new(1.0, Color32::from_gray(70)),
            window_highlight_topmost: true,

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(4),

            // === 面板样式 ===
            panel_fill: surface,

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [4, 8],
                blur: 10,
                spread: 0,
                color: Color32::from_black_alpha(60),
            },

            // === 其他组件样式 ===
            resize_corner_size: 10.0,
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, primary),
                preview: false,
                blink: true,
                on_duration: 0.6,
                off_duration: 0.4,
            },
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: false,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: false,
            handle_shape: HandleShape::Rect { aspect_ratio: 0.6 },
            interact_cursor: None,
            image_loading_spinners: true,
            numeric_color_space: NumericColorSpace::GammaByte,
            disabled_alpha: 0.6,
        },
        ..Default::default()
    }
}
