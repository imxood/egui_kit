//! Toast 通知系统
//!
//! 提供全局 Toast 通知支持，可以在任意位置发送，自动请求重绘。
//!
//! ## 使用方式
//!
//! ```rust
//! use egui_kit::toast::{GlobalToast};
//!
//! // 在 App 初始化时调用
//! GlobalToast::init(ctx);
//!
//! // 在任意位置发送 toast (不需要 ctx)
//! GlobalToast::success("操作成功");
//! GlobalToast::error("发生错误");
//!
//! // 在 App::update 中渲染
//! GlobalToast::render(ctx);
//! ```
//!
//! ## 在 Promise 回调中使用
//!
//! ```rust
//! use egui_kit::toast::GlobalToast;
//!
//! let promise = Promise::spawn(&app_ctx.runtime, ui.ctx(), async move {
//!     // 异步操作完成后发送 toast
//!     GlobalToast::success("保存成功");
//! });
//! ```

use std::sync::{Arc, Mutex, Weak};

use egui::{Align2, Context, Direction, RichText, Ui};
use egui_toast::{Toast as EToast, ToastKind as EToastKind, ToastOptions, Toasts};

/// Toast 通知级别
#[derive(Clone, Debug, PartialEq)]
pub enum ToastLevel {
    /// 信息提示
    Info,
    /// 成功提示
    Success,
    /// 警告提示
    Warning,
    /// 错误提示
    Error,
}

/// 待显示的 Toast 消息
#[derive(Clone, Debug)]
pub struct ToastMessage {
    pub level: ToastLevel,
    pub message: String,
    pub duration_secs: f32,
}

impl ToastMessage {
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            level: ToastLevel::Info,
            message: message.into(),
            duration_secs: 3.0,
        }
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self {
            level: ToastLevel::Success,
            message: message.into(),
            duration_secs: 3.0,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            level: ToastLevel::Error,
            message: message.into(),
            duration_secs: 4.0,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            level: ToastLevel::Warning,
            message: message.into(),
            duration_secs: 3.0,
        }
    }
}

/// 全局 Toast 状态
#[derive(Debug)]
struct GlobalToastState {
    /// egui Context 的弱引用
    ctx: Option<Weak<Context>>,
    /// 待显示的消息队列
    pending_toasts: Vec<ToastMessage>,
}

impl Default for GlobalToastState {
    fn default() -> Self {
        Self {
            ctx: None,
            pending_toasts: Vec::new(),
        }
    }
}

/// 全局 Toast 管理器
///
/// 使用 OnceLock 实现全局单例，支持在任何位置发送 toast。
#[derive(Debug, Clone)]
pub struct GlobalToast {
    state: Arc<Mutex<GlobalToastState>>,
}

impl Default for GlobalToast {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(GlobalToastState::default())),
        }
    }
}

impl GlobalToast {
    /// 初始化全局 Toast 系统
    ///
    /// 需要在 App 创建时调用，传入 egui Context。
    /// 由于 Context 不能直接存储在全局变量中，使用 Weak 引用。
    pub fn init(ctx: &Context) {
        let state = GlobalToast::get_or_init();
        let mut state = state.lock().unwrap_or_else(|e| e.into_inner());
        state.ctx = Some(Arc::downgrade(&Arc::new(ctx.clone())));
    }

    /// 获取或初始化全局状态
    fn get_or_init() -> &'static Arc<Mutex<GlobalToastState>> {
        use std::sync::OnceLock;
        static GLOBAL_STATE: OnceLock<Arc<Mutex<GlobalToastState>>> = OnceLock::new();
        GLOBAL_STATE.get_or_init(|| Arc::new(Mutex::new(GlobalToastState::default())))
    }

    /// 添加 Toast 消息到队列
    fn push_toast(message: ToastMessage) {
        let state = Self::get_or_init();
        let mut state = state.lock().unwrap_or_else(|e| e.into_inner());

        // 添加到队列
        state.pending_toasts.push(message);

        // 请求重绘
        if let Some(ctx) = &state.ctx {
            if let Some(ctx) = ctx.upgrade() {
                ctx.request_repaint();
            }
        }
    }

    /// 获取并清空待显示的消息
    pub fn consume_toasts() -> Vec<ToastMessage> {
        let state = Self::get_or_init();
        let mut state = state.lock().unwrap_or_else(|e| e.into_inner());
        state.pending_toasts.drain(..).collect()
    }

    /// 渲染所有待显示的 Toast
    pub fn render(ctx: &Context) {
        let messages = Self::consume_toasts();

        if messages.is_empty() {
            return;
        }

        // 创建 Toasts 管理器
        let mut toasts = Toasts::new()
            .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
            .direction(Direction::TopDown);

        // 添加每个 toast
        for message in messages {
            let toast = EToast {
                text: RichText::new(message.message).into(),
                kind: match message.level {
                    ToastLevel::Info => EToastKind::Info,
                    ToastLevel::Success => EToastKind::Success,
                    ToastLevel::Warning => EToastKind::Warning,
                    ToastLevel::Error => EToastKind::Error,
                },
                options: ToastOptions::default()
                    .duration_in_seconds(message.duration_secs as f64),
                ..Default::default()
            };
            toasts.add(toast);
        }

        toasts.show(ctx);
    }

    // ===== 便捷 API =====

    /// 添加信息提示
    pub fn info(message: impl Into<String>) {
        Self::push_toast(ToastMessage::info(message));
    }

    /// 添加成功提示
    pub fn success(message: impl Into<String>) {
        Self::push_toast(ToastMessage::success(message));
    }

    /// 添加错误提示
    pub fn error(message: impl Into<String>) {
        Self::push_toast(ToastMessage::error(message));
    }

    /// 添加警告提示
    pub fn warning(message: impl Into<String>) {
        Self::push_toast(ToastMessage::warning(message));
    }
}

/// Toast 通知管理器 (用于需要细粒度控制的场景)
#[derive(Default)]
pub struct Toast {
    toasts: Toasts,
}

impl Toast {
    /// 创建新的 Toast 管理器
    pub fn new() -> Self {
        Self {
            toasts: Toasts::new()
                .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
                .direction(Direction::TopDown),
        }
    }

    /// 获取 Toasts 的可变引用
    pub fn toasts_mut(&mut self) -> &mut Toasts {
        &mut self.toasts
    }

    /// 渲染 Toast 通知
    pub fn show(&mut self, ctx: &Context) {
        self.toasts.show(ctx);
    }

    /// 添加信息提示 (使用 Ui)
    pub fn info(ui: &mut Ui, message: impl Into<String>) {
        let mut toasts = Toasts::new()
            .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
            .direction(Direction::TopDown);
        toasts.add(EToast {
            text: RichText::new(message).into(),
            kind: EToastKind::Info,
            options: ToastOptions::default()
                .duration_in_seconds(3.0),
            ..Default::default()
        });
        toasts.show(ui.ctx());
    }

    /// 添加成功提示
    pub fn success(ui: &mut Ui, message: impl Into<String>) {
        let mut toasts = Toasts::new()
            .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
            .direction(Direction::TopDown);
        toasts.add(EToast {
            text: RichText::new(message).into(),
            kind: EToastKind::Success,
            options: ToastOptions::default()
                .duration_in_seconds(3.0),
            ..Default::default()
        });
        toasts.show(ui.ctx());
    }

    /// 添加错误提示
    pub fn error(ui: &mut Ui, message: impl Into<String>) {
        let mut toasts = Toasts::new()
            .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
            .direction(Direction::TopDown);
        toasts.add(EToast {
            text: RichText::new(message).into(),
            kind: EToastKind::Error,
            options: ToastOptions::default()
                .duration_in_seconds(4.0),
            ..Default::default()
        });
        toasts.show(ui.ctx());
    }

    /// 添加警告提示
    pub fn warning(ui: &mut Ui, message: impl Into<String>) {
        let mut toasts = Toasts::new()
            .anchor(Align2::RIGHT_TOP, (-10.0, 10.0))
            .direction(Direction::TopDown);
        toasts.add(EToast {
            text: RichText::new(message).into(),
            kind: EToastKind::Warning,
            options: ToastOptions::default()
                .duration_in_seconds(3.0),
            ..Default::default()
        });
        toasts.show(ui.ctx());
    }
}

/// 添加 Toast 到应用（使用 Context）
pub fn add_toast(ctx: &Context, toast: &mut Toast) {
    toast.show(ctx);
}

/// 添加信息提示的便捷函数
pub fn info(ui: &mut Ui, message: impl Into<String>) {
    Toast::info(ui, message);
}

/// 添加成功提示的便捷函数
pub fn success(ui: &mut Ui, message: impl Into<String>) {
    Toast::success(ui, message);
}

/// 添加错误提示的便捷函数
pub fn error(ui: &mut Ui, message: impl Into<String>) {
    Toast::error(ui, message);
}

/// 添加警告提示的便捷函数
pub fn warning(ui: &mut Ui, message: impl Into<String>) {
    Toast::warning(ui, message);
}
