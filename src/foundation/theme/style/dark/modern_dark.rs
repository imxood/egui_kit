//! Modern Dark 暗色主题
//!
//! 特点:
//! - 通用且专业的暗色设计
//! - 蓝色作为主色调，适中对比度
//! - 圆角: 6px, 阴影: 柔和
//!
//! 适用场景: 通用应用、工具软件

use egui::{Color32, Style, Visuals, Stroke, CornerRadius, Shadow};
use egui::style::{Widgets, WidgetVisuals, Selection, TextCursorStyle, HandleShape, NumericColorSpace};
use egui::epaint::AlphaFromCoverage;

/// 现代化暗色主题
///
/// # Style 字段说明
///
/// ## 基础模式设置
/// - `dark_mode`: true - 启用暗色模式
/// - `override_text_color`: None - 不覆盖文本颜色，使用默认
/// - `weak_text_alpha`: 0.6 - 弱文本透明度 (0-1)
/// - `weak_text_color`: None - 弱文本颜色，使用默认
/// - `text_alpha_from_coverage`: DARK_MODE_DEFAULT - 暗色模式默认文本覆盖率
///
/// ## 组件状态视觉 (widgets)
///
/// ### noninteractive: 不可交互状态
/// - 背景填充色: 深灰 (27)
/// - 弱背景填充: 同背景色
/// - 边框: 1px 浅灰 (60)
/// - 文字颜色: 1px 浅灰 (140)
/// - 圆角: 4px
/// - 扩张: 0.0 (无扩张效果)
///
/// ### inactive: 非激活状态 (未 Hover/Active)
/// - 背景填充: 中灰 (55)
/// - 弱背景填充: 同上
/// - 边框: 无边框
/// - 文字颜色: 1px 亮灰 (180)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ### hovered: 悬停状态
/// - 背景填充: 浅中灰 (65)
/// - 弱背景填充: 同上
/// - 边框: 1px 主色系浅灰 (100)
/// - 文字颜色: 1.5px 亮灰 (230)
/// - 圆角: 5px
/// - 扩张: 2.0 (悬停时组件轻微放大)
///
/// ### active: 激活状态 (按下)
/// - 背景填充: 中深灰 (50)
/// - 弱背景填充: 同上
/// - 边框: 1.5px 主色 (蓝色)
/// - 文字颜色: 2px 纯白
/// - 圆角: 4px
/// - 扩张: 1.0 (按下时轻微收缩)
///
/// ### open: 展开状态 (如下拉菜单)
/// - 背景填充: 表面色 (40)
/// - 弱背景填充: 深灰 (45)
/// - 边框: 1px 灰 (70)
/// - 文字颜色: 1px 灰 (200)
/// - 圆角: 4px
/// - 扩张: 0.0
///
/// ## 文本选择 (selection)
/// - 背景填充: 深蓝 (0, 92, 128, 透明度)
/// - 边框: 1px 亮蓝
///
/// ## 颜色配置
/// - `hyperlink_color`: 超链接颜色 (蓝色)
/// - `faint_bg_color`: 微弱背景色 (亮度加成 8)
/// - `extreme_bg_color`: 极端背景色 (接近纯黑 10)
/// - `text_edit_bg_color`: 文本编辑背景 (无，使用默认)
/// - `code_bg_color`: 代码块背景 (深灰 50)
/// - `warn_fg_color`: 警告色 (琥珀色)
/// - `error_fg_color`: 错误色 (红色)
///
/// ## 窗口样式 (window)
/// - 圆角: 8px
/// - 阴影: offset=[12,16], blur=20, alpha=100
/// - 填充色: 背景色
/// - 边框: 1px 浅灰 (60)
/// - 高亮最顶层: true
///
/// ## 菜单样式 (menu)
/// - 圆角: 6px
///
/// ## 面板样式 (panel)
/// - 填充色: 表面色 (40)
///
/// ## 弹出层样式 (popup)
/// - 阴影: offset=[8,12], blur=12, alpha=90
///
/// ## 其他组件
/// - `resize_corner_size`: 12.0 - 窗口大小调整角尺寸
/// - `text_cursor`: 文本光标样式 (2px 亮蓝, 闪烁 0.5s/0.5s)
/// - `clip_rect_margin`: 4.0 - 剪裁边距
/// - `button_frame`: true - 按钮有边框
/// - `collapsing_header_frame`: false - 可折叠标题无边框
/// - `indent_has_left_vline`: true - 缩进显示左边竖线
/// - `striped`: false - 表格无斑马纹
/// - `slider_trailing_fill`: false - 滑块尾部不填充
/// - `handle_shape`: Rect(0.75) - 滑块句柄矩形，长宽比 0.75
/// - `interact_cursor`: None - 交互时不自动切换光标
/// - `image_loading_spinners`: true - 图片加载显示旋转动画
/// - `numeric_color_space`: GammaByte - 数字颜色空间
/// - `disabled_alpha`: 0.5 - 禁用状态透明度
#[must_use]
pub fn modern_dark() -> Style {
    // 颜色定义
    let background = Color32::from_rgb(27, 27, 27);      // 背景色
    let surface = Color32::from_rgb(40, 40, 40);         // 表面色
    let primary = Color32::from_rgb(66, 133, 244);       // 主色 (蓝色)
    let warning = Color32::from_rgb(255, 193, 7);        // 警告色
    let error = Color32::from_rgb(244, 67, 54);          // 错误色
    let hyperlink = Color32::from_rgb(66, 133, 244);     // 超链接色

    Style {
        visuals: Visuals {
            // === 基础模式设置 ===
            dark_mode: true, // 启用暗色模式
            override_text_color: None, // 不覆盖文本颜色
            weak_text_alpha: 0.6, // 弱文本透明度 (次要文字)
            weak_text_color: None, // 弱文本颜色
            text_alpha_from_coverage: AlphaFromCoverage::DARK_MODE_DEFAULT, // 文本覆盖率计算方式

            // === 组件状态视觉 ===
            widgets: Widgets {
                // 不可交互状态 (如禁用按钮、只读输入框)
                noninteractive: WidgetVisuals {
                    bg_fill: Color32::from_gray(27), // 背景填充
                    weak_bg_fill: Color32::from_gray(27), // 弱背景填充 (用于次要元素)
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // 边框
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(140)), // 文字描边
                    corner_radius: CornerRadius::same(4), // 圆角半径
                    expansion: 0.0, // 扩张效果 (组件大小偏移)
                },
                // 非激活状态 (可点击但未交互)
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_gray(55),
                    weak_bg_fill: Color32::from_gray(55),
                    bg_stroke: Stroke::NONE, // 无边框
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(180)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
                // 悬停状态 (鼠标 hover)
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_gray(65),
                    weak_bg_fill: Color32::from_gray(65),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(100)),
                    fg_stroke: Stroke::new(1.5, Color32::from_gray(230)),
                    corner_radius: CornerRadius::same(5),
                    expansion: 2.0, // 悬停时组件略微放大
                },
                // 激活状态 (鼠标按下)
                active: WidgetVisuals {
                    bg_fill: Color32::from_gray(50),
                    weak_bg_fill: Color32::from_gray(50),
                    bg_stroke: Stroke::new(1.5, primary),
                    fg_stroke: Stroke::new(2.0, Color32::WHITE),
                    corner_radius: CornerRadius::same(4),
                    expansion: 1.0, // 按下时组件略微收缩
                },
                // 展开状态 (如下拉菜单、折叠面板)
                open: WidgetVisuals {
                    bg_fill: surface,
                    weak_bg_fill: Color32::from_gray(45),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(70)),
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(200)),
                    corner_radius: CornerRadius::same(4),
                    expansion: 0.0,
                },
            },

            // === 文本选择 ===
            selection: Selection {
                bg_fill: Color32::from_rgb(0, 92, 128), // 选择背景 (深蓝色)
                stroke: Stroke::new(1.0, Color32::from_rgb(192, 222, 255)), // 选择边框
            },

            // === 颜色配置 ===
            hyperlink_color: hyperlink, // 超链接颜色
            faint_bg_color: Color32::from_additive_luminance(8), // 微弱背景色 (加法亮度)
            extreme_bg_color: Color32::from_gray(10), // 极端背景色 (最深色)
            text_edit_bg_color: None, // 文本编辑背景 (None=使用默认)
            code_bg_color: Color32::from_gray(50), // 代码块背景
            warn_fg_color: warning, // 警告文字颜色
            error_fg_color: error, // 错误文字颜色

            // === 窗口样式 ===
            window_corner_radius: CornerRadius::same(8), // 窗口圆角
            window_shadow: Shadow {
                offset: [12, 16], // 阴影偏移 (x, y)
                blur: 20, // 模糊半径
                spread: 0, // 扩散半径
                color: Color32::from_black_alpha(100), // 阴影颜色 (不透明度)
            },
            window_fill: background, // 窗口填充色
            window_stroke: Stroke::new(1.0, Color32::from_gray(60)), // 窗口边框
            window_highlight_topmost: true, // 高亮最顶层窗口

            // === 菜单样式 ===
            menu_corner_radius: CornerRadius::same(6), // 菜单圆角

            // === 面板样式 ===
            panel_fill: surface, // 面板填充色

            // === 弹出层样式 ===
            popup_shadow: Shadow {
                offset: [8, 12],
                blur: 12,
                spread: 0,
                color: Color32::from_black_alpha(90),
            },

            // === 其他组件样式 ===
            resize_corner_size: 12.0, // 窗口调整角大小
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(2.0, Color32::from_rgb(192, 222, 255)), // 光标描边
                preview: false, // 是否预览
                blink: true, // 是否闪烁
                on_duration: 0.5, // 亮起持续时间 (秒)
                off_duration: 0.5, // 熄灭持续时间 (秒)
            },
            clip_rect_margin: 4.0, // 剪裁矩形边距
            button_frame: true, // 按钮是否绘制边框
            collapsing_header_frame: false, // 折叠标题是否绘制边框
            indent_has_left_vline: true, // 缩进层级是否显示左边竖线
            striped: false, // 表格是否启用斑马纹
            slider_trailing_fill: false, // 滑块是否填充尾部
            handle_shape: HandleShape::Rect { aspect_ratio: 0.75 }, // 滑块句柄形状
            interact_cursor: None, // 交互时是否自动切换光标
            image_loading_spinners: true, // 图片加载是否显示旋转动画
            numeric_color_space: NumericColorSpace::GammaByte, // 数字颜色空间
            disabled_alpha: 0.5, // 禁用状态透明度
        },
        ..Default::default()
    }
}
