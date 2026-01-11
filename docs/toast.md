# Toast 通知系统

**最后更新:** 2026-01-10

---

## 模块概述

- **功能:** 提供全局 Toast 通知支持，可在任意位置发送通知
- **依赖:** `egui_toast` crate
- **核心文件:** `src/toast.rs`

## 功能列表

### GlobalToast 全局通知
- **核心函数:** `GlobalToast::init()`, `GlobalToast::success()`, `GlobalToast::info()`, `GlobalToast::warning()`, `GlobalToast::error()`, `GlobalToast::render()`
- **功能特性:** 线程安全的全局通知系统

### Toast 本地通知
- **核心函数:** `Toast::info()`, `Toast::success()`, `Toast::warning()`, `Toast::error()`
- **功能特性:** 需要传入 `&mut Ui` 参数

## 业务流程

### GlobalToast 使用流程
```
初始化 → 任意位置调用 → update中渲染
```
1. 初始化: `GlobalToast::init(ctx)` 在 App 创建时调用
2. 调用: `GlobalToast::success("message")` 不需要 ctx
3. 渲染: `GlobalToast::render(ctx)` 在 App::update 中调用

## 公共函数库

### GlobalToast 静态方法
```rust
// 初始化 (在 App 创建时调用)
GlobalToast::init(ctx: &Context);

// 发送通知 (可在任意位置调用, 包括异步回调)
GlobalToast::info(message: impl Into<String>);
GlobalToast::success(message: impl Into<String>);
GlobalToast::warning(message: impl Into<String>);
GlobalToast::error(message: impl Into<String>);

// 渲染 (在 App::update 中调用)
GlobalToast::render(ctx: &Context);
```

### Toast 实例方法
```rust
// 创建 Toast 管理器
let mut toast = Toast::new();

// 添加通知 (需要 &mut Ui)
toast.info(ui, "message");
toast.success(ui, "message");
toast.warning(ui, "message");
toast.error(ui, "message");

// 渲染
toast.show(ctx);
```

## 关键数据结构

### ToastLevel 通知级别
```rust
#[derive(Clone, Debug, PartialEq)]
pub enum ToastLevel {
    Info,      // 信息提示
    Success,   // 成功提示
    Warning,   // 警告提示
    Error,     // 错误提示
}
```

### ToastMessage 待显示消息
```rust
#[derive(Clone, Debug)]
pub struct ToastMessage {
    pub level: ToastLevel,
    pub message: String,
    pub duration_secs: f32,
}
```

## 配置参数

- 默认显示时长: Info/Warning/Success = 3秒, Error = 4秒
- 显示位置: 右上角 (Align2::RIGHT_TOP)

---

**维护说明:**
- 修改 Toast 通知行为时, 更新功能列表章节
- 修改显示位置/时长时, 更新配置参数章节
