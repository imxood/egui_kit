//! Dialog - 定制对话框组件
//!
//! 支持两种类型:
//! - `Text`: 普通文本对话框 (不可取消)
//! - `Cancellable`: 可取消对话框 (带取消按钮和回调)
//!
//! # 使用示例
//!
//! ```rust,no_run
//! use egui_kit::dialog::Dialog;
//!
//! // 普通文本
//! Dialog::show_text(&ctx, "loading", "加载中...");
//! Dialog::close(&ctx, "loading");
//!
//! // 可取消 (带数据)
//! Dialog::show_cancellable(
//!     &ctx,
//!     "download",
//!     "下载中...",
//!     "取消",
//!     task_id,
//!     |id| { cancel_download(id); },
//! );
//!
//! // 渲染 (在 update 末尾调用)
//! Dialog::render(&ctx);
//! ```

use std::any::Any;
use std::sync::Arc;

use egui::{Align2, Color32, Context, Id, Order, Pos2, Sense};
use indexmap::IndexMap;

/// 取消回调 (带用户数据)
pub type CancelCallback = Arc<dyn Fn(Box<dyn Any + Send + Sync>) + Send + Sync>;

/// 对话框内部数据 (用于存储回调和用户数据)
struct DialogData {
    user_data: Box<dyn Any + Send + Sync>,
    on_cancel: CancelCallback,
}

/// 对话框渲染信息 (可 Clone, 用于渲染)
#[derive(Clone)]
struct DialogRender {
    message: String,
    show_spinner: bool,
    cancel_text: Option<String>,
    size: Option<egui::Vec2>,  // None = 自适应
}

/// 对话框状态 (存储在 egui memory)
#[derive(Default, Clone)]
struct DialogState {
    /// 渲染信息 (Arc 包装, 读取时 cheap clone)
    renders: Arc<IndexMap<String, DialogRender>>,
    /// 待取消的 key
    pending_cancel: Option<String>,
}

/// 对话框数据存储 (全局静态, 存储回调)
static DIALOG_DATA: std::sync::OnceLock<parking_lot::Mutex<IndexMap<String, DialogData>>> =
    std::sync::OnceLock::new();

fn dialog_data() -> &'static parking_lot::Mutex<IndexMap<String, DialogData>> {
    DIALOG_DATA.get_or_init(|| parking_lot::Mutex::new(IndexMap::new()))
}

/// 对话框 API
pub struct Dialog;

impl Dialog {
    /// 获取状态 (从 egui memory)
    fn with_state<R>(ctx: &Context, f: impl FnOnce(&mut DialogState) -> R) -> R {
        ctx.memory_mut(|mem| f(mem.data.get_temp_mut_or_default::<DialogState>(Id::NULL)))
    }

    // ==================== 公开 API ====================

    /// 显示普通文本对话框 (不可取消)
    pub fn show_text(ctx: &Context, key: impl Into<String>, message: impl Into<String>) {
        Self::show_text_impl(ctx, key, message, true, None);
    }

    /// 显示普通文本对话框 (自定义大小)
    pub fn show_text_with_size(
        ctx: &Context,
        key: impl Into<String>,
        message: impl Into<String>,
        size: egui::Vec2,
    ) {
        Self::show_text_impl(ctx, key, message, true, Some(size));
    }

    /// 显示普通文本对话框 (可配置 spinner)
    pub fn show_text_with_spinner(
        ctx: &Context,
        key: impl Into<String>,
        message: impl Into<String>,
        show_spinner: bool,
    ) {
        Self::show_text_impl(ctx, key, message, show_spinner, None);
    }

    /// 内部实现
    fn show_text_impl(
        ctx: &Context,
        key: impl Into<String>,
        message: impl Into<String>,
        show_spinner: bool,
        size: Option<egui::Vec2>,
    ) {
        let key = key.into();
        Self::with_state(ctx, |state| {
            Arc::make_mut(&mut state.renders).insert(
                key,
                DialogRender {
                    message: message.into(),
                    show_spinner,
                    cancel_text: None,
                    size,
                },
            );
        });
    }

    /// 显示可取消对话框 (带用户数据)
    pub fn show_cancellable<T, F>(
        ctx: &Context,
        key: impl Into<String>,
        message: impl Into<String>,
        cancel_text: impl Into<String>,
        user_data: T,
        on_cancel: F,
    ) where
        T: Send + Sync + 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        Self::show_cancellable_impl(ctx, key, message, cancel_text, true, None, user_data, on_cancel);
    }

    /// 显示可取消对话框 (自定义大小)
    pub fn show_cancellable_with_size<T, F>(
        ctx: &Context,
        key: impl Into<String>,
        message: impl Into<String>,
        cancel_text: impl Into<String>,
        size: egui::Vec2,
        user_data: T,
        on_cancel: F,
    ) where
        T: Send + Sync + 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        Self::show_cancellable_impl(ctx, key, message, cancel_text, true, Some(size), user_data, on_cancel);
    }

    /// 显示可取消对话框 (可配置 spinner)
    pub fn show_cancellable_with_spinner<T, F>(
        ctx: &Context,
        key: impl Into<String>,
        message: impl Into<String>,
        cancel_text: impl Into<String>,
        show_spinner: bool,
        user_data: T,
        on_cancel: F,
    ) where
        T: Send + Sync + 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        Self::show_cancellable_impl(ctx, key, message, cancel_text, show_spinner, None, user_data, on_cancel);
    }

    /// 内部实现
    fn show_cancellable_impl<T, F>(
        ctx: &Context,
        key: impl Into<String>,
        message: impl Into<String>,
        cancel_text: impl Into<String>,
        show_spinner: bool,
        size: Option<egui::Vec2>,
        user_data: T,
        on_cancel: F,
    ) where
        T: Send + Sync + 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        let key = key.into();
        let cancel_text = cancel_text.into();

        // 存储渲染信息
        Self::with_state(ctx, |state| {
            Arc::make_mut(&mut state.renders).insert(
                key.clone(),
                DialogRender {
                    message: message.into(),
                    show_spinner,
                    cancel_text: Some(cancel_text),
                    size,
                },
            );
        });

        // 存储回调数据
        dialog_data().lock().insert(
            key,
            DialogData {
                user_data: Box::new(user_data),
                on_cancel: Arc::new(move |data| {
                    if let Ok(typed) = data.downcast::<T>() {
                        on_cancel(*typed);
                    }
                }),
            },
        );
    }

    /// 关闭指定对话框
    pub fn close(ctx: &Context, key: impl AsRef<str>) {
        let key = key.as_ref();
        Self::with_state(ctx, |state| {
            Arc::make_mut(&mut state.renders).shift_remove(key);
        });
        dialog_data().lock().shift_remove(key);
    }

    /// 关闭所有对话框
    pub fn close_all(ctx: &Context) {
        Self::with_state(ctx, |state| {
            Arc::make_mut(&mut state.renders).clear();
        });
        dialog_data().lock().clear();
    }

    /// 检查是否有对话框显示
    pub fn is_showing(ctx: &Context) -> bool {
        Self::with_state(ctx, |state| !state.renders.is_empty())
    }

    /// 检查指定对话框是否存在
    pub fn has(ctx: &Context, key: impl AsRef<str>) -> bool {
        Self::with_state(ctx, |state| state.renders.contains_key(key.as_ref()))
    }

    /// 渲染所有对话框 (在 update 末尾调用)
    pub fn render(ctx: &Context) {
        // 处理上一帧的取消请求
        let pending = Self::with_state(ctx, |state| state.pending_cancel.take());

        if let Some(key) = pending {
            // 移除渲染信息
            Self::with_state(ctx, |state| {
                Arc::make_mut(&mut state.renders).shift_remove(&key);
            });

            // 执行回调
            if let Some(data) = dialog_data().lock().shift_remove(&key) {
                (data.on_cancel)(data.user_data);
            }
        }

        // 获取渲染数据 (只 clone Arc 指针, cheap)
        let renders = Self::with_state(ctx, |state| state.renders.clone());

        // 记录本帧点击的取消按钮
        let mut clicked_cancel: Option<String> = None;

        for (key, render) in renders.iter() {
            if Self::render_one(ctx, key, render) {
                clicked_cancel = Some(key.clone());
            }
        }

        // 设置 pending_cancel (下一帧处理)
        if let Some(key) = clicked_cancel {
            Self::with_state(ctx, |state| {
                state.pending_cancel = Some(key);
            });
        }
    }

    /// 渲染单个对话框, 返回是否点击了取消按钮
    fn render_one(ctx: &Context, key: &str, render: &DialogRender) -> bool {
        let mut cancelled = false;

        // 对话框 (包含遮罩和内容, 避免焦点切换)
        egui::Area::new(Id::new(format!("dialog_{}", key)))
            .fixed_pos(Pos2::ZERO)
            .order(Order::Foreground)
            .interactable(true)
            .show(ctx, |ui| {
                let screen = ctx.input(|i| i.content_rect());

                // 绘制半透明遮罩
                ui.painter()
                    .rect_filled(screen, 0.0, Color32::from_black_alpha(128));

                // 绘制对话框内容 (居中)
                let center = screen.center();

                if let Some(size) = render.size {
                    // 固定大小
                    let content_rect = egui::Rect::from_center_size(center, size);
                    ui.allocate_ui_at_rect(content_rect, |ui| {
                        cancelled = Self::render_dialog_content(ui, render);
                    });
                } else {
                    // 自适应大小
                    ui.allocate_ui_at_rect(
                        egui::Rect::from_center_size(center, egui::vec2(300.0, 100.0)),
                        |ui| {
                            egui::Frame::popup(ui.style())
                                .fill(ui.style().visuals.window_fill)
                                .inner_margin(egui::Margin::same(24))
                                .show(ui, |ui| {
                                    ui.vertical_centered(|ui| {
                                        if render.show_spinner {
                                            ui.spinner();
                                            ui.add_space(12.0);
                                        }

                                        ui.label(&render.message);

                                        if let Some(cancel_text) = &render.cancel_text {
                                            ui.add_space(16.0);
                                            if ui.button(cancel_text).clicked() {
                                                cancelled = true;
                                            }
                                        }
                                    });
                                });
                        },
                    );
                }
            });

        cancelled
    }

    /// 渲染对话框内容 (固定大小模式)
    fn render_dialog_content(ui: &mut egui::Ui, render: &DialogRender) -> bool {
        let mut cancelled = false;

        egui::Frame::popup(ui.style())
            .fill(ui.style().visuals.window_fill)
            .inner_margin(egui::Margin::same(24))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    if render.show_spinner {
                        ui.spinner();
                        ui.add_space(12.0);
                    }

                    ui.label(&render.message);

                    if let Some(cancel_text) = &render.cancel_text {
                        ui.add_space(16.0);
                        if ui.button(cancel_text).clicked() {
                            cancelled = true;
                        }
                    }
                });
            });

        cancelled
    }
}
