//! Promise - Async operation wrapper for egui
//!
//! Simplifies async result handling in immediate mode UI
//! Supports cancellation for long-running operations (BLE connect, OTA, etc.)

use tokio::runtime::Handle;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

pub type MaybePromise<T> = Option<Promise<T>>;

/// Promise for async operations
///
/// Wraps a future and provides non-blocking result polling with cancellation support
pub struct Promise<T> {
    receiver: oneshot::Receiver<T>,
    task_handle: JoinHandle<()>,
}

impl<T> Promise<T> {
    /// Create a Promise from a future
    ///
    /// # Example
    /// ```ignore
    /// let mut promise = Promise::new(&runtime, async {
    ///     session.connect(&address).await
    /// });
    ///
    /// // Later, if needed:
    /// promise.cancel();
    /// ```
    pub fn new<F>(runtime: &Handle, future: F) -> Self
    where
        F: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        let task_handle = runtime.spawn(async move {
            let result = future.await;
            let _ = tx.send(result);
        });

        Self {
            receiver: rx,
            task_handle,
        }
    }

    /// Poll for result (non-blocking)
    ///
    /// Returns Some(result) if ready, None if still pending or cancelled
    pub fn poll(&mut self) -> Option<T> {
        match self.receiver.try_recv() {
            Ok(result) => Some(result),
            Err(oneshot::error::TryRecvError::Empty) => None, // Still running
            Err(oneshot::error::TryRecvError::Closed) => None, // Cancelled
        }
    }

    /// Check if the promise is ready
    pub fn is_ready(&mut self) -> bool {
        matches!(
            self.receiver.try_recv(),
            Ok(_) | Err(oneshot::error::TryRecvError::Closed)
        )
    }

    /// Cancel the running task
    ///
    /// Aborts the underlying async task immediately.
    /// Useful for long-running operations like BLE connection or OTA.
    /// This method is idempotent (can be called multiple times safely).
    ///
    /// # Example
    /// ```ignore
    /// let mut promise = Promise::new(&runtime, async {
    ///     // Long-running BLE connection
    ///     session.connect(&address).await
    /// });
    ///
    /// // User clicked cancel button
    /// promise.cancel();
    /// ```
    pub fn cancel(&self) {
        self.task_handle.abort();
    }

    /// Check if the task has been cancelled or finished
    pub fn is_cancelled(&self) -> bool {
        self.task_handle.is_finished()
    }
}

impl<T> Drop for Promise<T> {
    fn drop(&mut self) {
        // Auto-cancel unfinished tasks to prevent resource leaks
        self.task_handle.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_promise_success() {
        let runtime = tokio::runtime::Handle::current();
        let mut promise = Promise::new(&runtime, async { 42 });

        // Initially not ready
        assert!(promise.poll().is_none());

        // Wait a bit
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Should be ready
        assert_eq!(promise.poll(), Some(42));
    }

    #[tokio::test]
    async fn test_promise_with_result() {
        let runtime = tokio::runtime::Handle::current();
        let mut promise = Promise::new(&runtime, async { Ok::<_, String>(100) });

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        match promise.poll() {
            Some(Ok(value)) => assert_eq!(value, 100),
            _ => panic!("Expected Ok(100)"),
        }
    }

    #[tokio::test]
    async fn test_promise_cancel() {
        use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

        let runtime = tokio::runtime::Handle::current();

        // 使用原子布尔值跟踪任务是否完成
        let task_completed = Arc::new(AtomicBool::new(false));
        let task_completed_clone = task_completed.clone();

        let mut promise = Promise::new(&runtime, async move {
            // 模拟长时间运行的任务 (5秒)
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            // 如果执行到这里, 说明任务没有被取消
            task_completed_clone.store(true, Ordering::SeqCst);
            eprintln!("❌ Task completed - this should NOT print!");

            42
        });

        // 初始状态检查
        assert!(promise.poll().is_none(), "Promise should not be ready initially");
        assert!(!promise.is_cancelled(), "Promise should not be cancelled initially");
        assert!(!task_completed.load(Ordering::SeqCst), "Task should not be completed initially");

        // 等待 2 秒 (任务还在运行)
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // 取消任务
        eprintln!("✓ Cancelling task after 2 seconds");
        promise.cancel();

        // 等待足够长的时间, 确保任务如果没被取消会执行完毕 (再等3秒, 总共5秒)
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        // 验证任务被成功取消
        assert!(promise.is_cancelled(), "Promise should be cancelled");
        assert!(promise.poll().is_none(), "Cancelled promise should return None");
        assert!(!task_completed.load(Ordering::SeqCst), "Task should NOT have completed after cancellation");

        eprintln!("✓ Task was successfully cancelled - no output from task");
    }

    #[tokio::test]
    async fn test_promise_auto_cancel_on_drop() {
        use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

        let runtime = tokio::runtime::Handle::current();

        // 使用原子布尔值跟踪任务是否完成
        let task_completed = Arc::new(AtomicBool::new(false));
        let task_completed_clone = task_completed.clone();

        {
            let _promise = Promise::new(&runtime, async move {
                // 模拟长时间运行的任务 (5秒)
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                // 如果执行到这里, 说明任务没有被自动取消
                task_completed_clone.store(true, Ordering::SeqCst);
                println!("❌ Task completed in drop test - this should NOT print!");

                42
            });

            // 等待 1 秒后 drop Promise
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("✓ Dropping promise after 1 second");

            // Promise 在这里被 drop, 应该自动取消任务
        }

        // 再等待 5 秒, 确保任务如果没被取消会执行完毕
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // 验证任务被自动取消
        assert!(!task_completed.load(Ordering::SeqCst), "Task should NOT have completed after promise dropped");

        println!("✓ Task was auto-cancelled on drop - no resource leak");
    }
}
