//! MaybePromise - 异步操作封装
//!
//! 简化 egui 即时模式中的异步处理:
//! - 原位发起请求
//! - 每帧轮询结果
//! - 自动触发重绘
//! - 支持取消异步任务
//!
//! 用法:
//! ```ignore
//! use iot_tool_web::runtime;
//!
//! struct MyState {
//!     load_promise: MaybePromise<Result<Data, Error>>,
//! }
//!
//! // 发起请求 (统一接口)
//! state.load_promise = Some(Promise::spawn(&runtime(), ctx, async {
//!     api::load().await
//! }));
//!
//! // 轮询结果
//! if let Some(promise) = &mut state.load_promise {
//!     if let Some(result) = promise.poll() {
//!         // 处理结果
//!         state.load_promise = None;
//!     }
//! }
//!
//! // 主动取消
//! if let Some(promise) = &state.load_promise {
//!     promise.cancel();
//! }
//! state.load_promise = None;  // Drop 时自动取消
//! ```

use std::future::Future;
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::oneshot;
#[cfg(not(target_arch = "wasm32"))]
use tokio_util::sync::CancellationToken;

/// MaybePromise 类型别名
pub type MaybePromise<T> = Option<Promise<T>>;

/// 异步操作封装 (实现 Clone 以支持 egui::Memory 存储)
#[derive(Clone)]
pub struct Promise<T> {
    #[cfg(not(target_arch = "wasm32"))]
    inner: Arc<Mutex<PromiseInner<T>>>,
    #[cfg(not(target_arch = "wasm32"))]
    cancel_token: CancellationToken,

    #[cfg(target_arch = "wasm32")]
    inner: Arc<Mutex<PromiseState<T>>>,
}

#[cfg(not(target_arch = "wasm32"))]
struct PromiseInner<T> {
    receiver: Option<oneshot::Receiver<T>>,
    result: Option<T>,
}

#[cfg(target_arch = "wasm32")]
struct PromiseState<T> {
    result: Option<T>,
    cancelled: bool,
}

impl<T: Send + 'static> Promise<T> {
    /// 创建可取消的 Promise (Native)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn spawn<F>(runtime: &tokio::runtime::Handle, ctx: &egui::Context, future: F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let ctx_clone = ctx.clone();
        let cancel_token = CancellationToken::new();
        let cancel_token_clone = cancel_token.clone();

        runtime.spawn(async move {
            tokio::select! {
                result = future => {
                    let _ = tx.send(result);
                    ctx_clone.request_repaint();
                }
                _ = cancel_token_clone.cancelled() => {
                    // 任务被取消，不发送结果
                    log::debug!("Promise cancelled");
                }
            }
        });

        Self {
            inner: Arc::new(Mutex::new(PromiseInner {
                receiver: Some(rx),
                result: None,
            })),
            cancel_token,
        }
    }

    /// 创建可取消的 Promise (WASM)
    #[cfg(target_arch = "wasm32")]
    pub fn spawn<F>(_runtime: &(), ctx: &egui::Context, future: F) -> Self
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        let inner = Arc::new(Mutex::new(PromiseState {
            result: None,
            cancelled: false,
        }));
        let inner_clone = inner.clone();
        let ctx = ctx.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let result = future.await;
            let mut state = inner_clone.lock().unwrap();
            // 只在未取消时存储结果并触发重绘
            if !state.cancelled {
                state.result = Some(result);
                ctx.request_repaint();
            }
        });

        Self { inner }
    }

    /// 取消异步任务 (Native)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    /// 取消异步任务 (WASM)
    #[cfg(target_arch = "wasm32")]
    pub fn cancel(&self) {
        let mut state = self.inner.lock().unwrap();
        state.cancelled = true;
        state.result = None;  // 清空可能已有的结果
    }

    /// 非阻塞轮询结果
    pub fn poll(&mut self) -> Option<T> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut guard = self.inner.lock().unwrap();
            // 如果已有结果，返回
            if guard.result.is_some() {
                return guard.result.take();
            }
            // 尝试接收
            if let Some(rx) = &mut guard.receiver {
                match rx.try_recv() {
                    Ok(result) => {
                        guard.receiver = None;
                        return Some(result);
                    }
                    Err(oneshot::error::TryRecvError::Empty) => {}
                    Err(oneshot::error::TryRecvError::Closed) => {
                        guard.receiver = None;
                    }
                }
            }
            None
        }

        #[cfg(target_arch = "wasm32")]
        {
            let mut state = self.inner.lock().unwrap();
            // 如果已取消，返回 None
            if state.cancelled {
                return None;
            }
            state.result.take()
        }
    }

    /// 检查是否已完成
    #[cfg(not(target_arch = "wasm32"))]
    pub fn is_ready(&self) -> bool {
        let guard = self.inner.lock().unwrap();
        guard.result.is_some() || guard.receiver.is_none()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn is_ready(&self) -> bool {
        let state = self.inner.lock().unwrap();
        state.result.is_some() && !state.cancelled
    }

    /// 检查是否已取消
    #[cfg(not(target_arch = "wasm32"))]
    pub fn is_cancelled(&self) -> bool {
        self.cancel_token.is_cancelled()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn is_cancelled(&self) -> bool {
        self.inner.lock().unwrap().cancelled
    }
}

// 实现 Drop trait，自动取消异步任务
#[cfg(not(target_arch = "wasm32"))]
impl<T> Drop for Promise<T> {
    fn drop(&mut self) {
        // 只有最后一个引用被 drop 时才真正取消
        if Arc::strong_count(&self.inner) == 1 {
            self.cancel_token.cancel();
            log::debug!("async task done");
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl<T> Drop for Promise<T> {
    fn drop(&mut self) {
        // 只有最后一个引用被 drop 时才真正取消
        if Arc::strong_count(&self.inner) == 1 {
            let mut state = self.inner.lock().unwrap();
            state.cancelled = true;
            state.result = None;
            log::debug!("async task done");
        }
    }
}

/// 组件状态宏 - 简化 egui::Memory 访问
///
/// 用法:
/// ```ignore
/// #[derive(Default)]
/// pub struct MyComponentState {
///     pub data: Vec<Item>,
///     pub load_promise: MaybePromise<Result<Vec<Item>, Error>>,
/// }
/// define_component_state!(MyComponentState, "my_component");
///
/// // 在 UI 中使用
/// MyComponentState::with(ctx, |state| {
///     // 访问和修改 state
/// });
/// ```
#[macro_export]
macro_rules! define_component_state {
    ($name:ident, $id:expr) => {
        impl $name {
            /// 使用闭包访问组件状态
            pub fn with<R>(ctx: &egui::Context, f: impl FnOnce(&mut Self) -> R) -> R {
                ctx.memory_mut(|m| {
                    let state = m.data.get_temp_mut_or_default::<Self>(egui::Id::new($id));
                    f(state)
                })
            }

            /// 设置组件状态
            pub fn set(ctx: &egui::Context, state: Self) {
                ctx.memory_mut(|m| {
                    m.data.insert_temp(egui::Id::new($id), state);
                });
            }
        }
    };
}

// ==================== TimedPromise 超时封装 ====================

use std::time::{Duration, Instant};

/// 默认超时时间: 30 秒
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// TimedPromise 类型别名
pub type MaybeTimedPromise<T> = Option<TimedPromise<T>>;

/// Promise 超时状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeoutStatus {
    /// 正在进行中
    InProgress,
    /// 已完成
    Completed,
    /// 已超时
    TimedOut,
}

/// 带超时的 Promise 包装器
///
/// 为异步请求提供超时检测功能, 避免无限等待
///
/// 用法:
/// ```ignore
/// // 创建带超时的 Promise
/// let promise = Promise::spawn(&runtime, ctx, async { ... });
/// let timed = TimedPromise::new(promise);
///
/// // 轮询 (自动检测超时)
/// match timed.poll() {
///     Some(Ok(result)) => { /* 成功 */ }
///     Some(Err(e)) => { /* 错误或超时 */ }
///     None => { /* 仍在进行中 */ }
/// }
///
/// // 检查是否超时
/// if timed.is_timed_out() {
///     // 显示超时提示
/// }
/// ```
#[derive(Clone)]
pub struct TimedPromise<T> {
    /// 内部 Promise
    promise: Promise<T>,
    /// 开始时间
    started_at: Instant,
    /// 超时时长
    timeout: Duration,
    /// 是否已标记超时
    timed_out: Arc<Mutex<bool>>,
}

impl<T: Send + 'static> TimedPromise<T> {
    /// 创建新的 TimedPromise (使用默认超时 30 秒)
    pub fn new(promise: Promise<T>) -> Self {
        Self {
            promise,
            started_at: Instant::now(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            timed_out: Arc::new(Mutex::new(false)),
        }
    }

    /// 创建新的 TimedPromise (指定超时秒数)
    pub fn with_timeout(promise: Promise<T>, timeout_secs: u64) -> Self {
        Self {
            promise,
            started_at: Instant::now(),
            timeout: Duration::from_secs(timeout_secs),
            timed_out: Arc::new(Mutex::new(false)),
        }
    }

    /// 轮询 Promise 结果 (自动检测超时)
    ///
    /// 返回:
    /// - `Some(Ok(result))`: Promise 已完成
    /// - `Some(Err(error))`: Promise 出错或超时
    /// - `None`: 仍在进行中
    pub fn poll(&mut self) -> Option<Result<T, String>>
    where
        T: Clone,
    {
        // 如果已经标记超时, 返回超时错误
        {
            let timed_out = self.timed_out.lock().unwrap();
            if *timed_out {
                return Some(Err("请求超时".to_string()));
            }
        }

        // 检查是否超时
        if self.started_at.elapsed() > self.timeout {
            let mut timed_out = self.timed_out.lock().unwrap();
            *timed_out = true;
            return Some(Err(format!(
                "请求超时 (超过 {} 秒)",
                self.timeout.as_secs()
            )));
        }

        // 正常轮询内部 Promise
        self.promise.poll().map(Ok)
    }

    /// 获取当前状态
    pub fn status(&self) -> TimeoutStatus {
        let timed_out = self.timed_out.lock().unwrap();
        if *timed_out {
            return TimeoutStatus::TimedOut;
        }
        if self.started_at.elapsed() > self.timeout {
            return TimeoutStatus::TimedOut;
        }
        if self.promise.is_ready() {
            return TimeoutStatus::Completed;
        }
        TimeoutStatus::InProgress
    }

    /// 获取已等待时间 (秒)
    pub fn elapsed_secs(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }

    /// 获取超时时间 (秒)
    pub fn timeout_secs(&self) -> u64 {
        self.timeout.as_secs()
    }

    /// 是否已超时
    pub fn is_timed_out(&self) -> bool {
        let timed_out = self.timed_out.lock().unwrap();
        *timed_out || self.started_at.elapsed() > self.timeout
    }

    /// 取消内部 Promise
    pub fn cancel(&self) {
        self.promise.cancel();
    }

    /// 获取内部 Promise 的引用
    pub fn inner(&self) -> &Promise<T> {
        &self.promise
    }
}

