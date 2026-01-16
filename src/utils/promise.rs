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
