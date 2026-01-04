/*!
egui_kit 日志组件

功能:
- 实现 log crate 的 Log trait
- 使用 egui memory 缓存日志 (每个 Context 独立)
- 提供 LogPanel UI 组件显示日志

使用方式:
```rust
use egui_kit::logger::{init_logger, LogPanel};

// 1. 初始化日志 (程序启动时调用一次)
init_logger().ok();

// 2. 在 UI 中显示日志面板
LogPanel::new()
    .max_entries(500)
    .show(ctx, |response| {
        // 可选: 处理面板响应
    });
```
*/

use std::collections::VecDeque;
use std::sync::LazyLock;

use chrono::Local;
use crossbeam_channel::{Receiver, Sender, unbounded};
use egui::{Color32, Id, RichText, ScrollArea, Ui};
use log::{Level, Metadata, Record};

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// 时间戳字符串 (格式: [12-31 13:47:04.020])
    pub timestamp: String,
    /// 日志级别
    pub level: Level,
    /// 日志消息
    pub message: String,
}

impl LogEntry {
    /// 创建新日志条目
    pub fn new(level: Level, message: String) -> Self {
        let timestamp = Local::now().format("[%m-%d %H:%M:%S%.3f]").to_string();
        Self {
            timestamp,
            level,
            message,
        }
    }

    /// 获取完整日志行 (带时间戳)
    pub fn full_line(&self) -> String {
        format!("{} {}", self.timestamp, self.message)
    }

    /// 获取级别颜色
    pub fn level_color(&self) -> Color32 {
        match self.level {
            Level::Error => Color32::from_rgb(255, 100, 100), // 红色
            Level::Warn => Color32::from_rgb(255, 200, 100),  // 橙色
            Level::Info => Color32::from_rgb(150, 200, 255),  // 蓝色
            Level::Debug => Color32::from_rgb(180, 180, 180), // 灰色
            Level::Trace => Color32::from_rgb(150, 150, 150), // 浅灰
        }
    }
}

// ============================================================================
// 全局日志通道 (用于从任意线程发送日志)
// ============================================================================

static LOG_CHANNEL: LazyLock<(Sender<LogEntry>, Receiver<LogEntry>)> =
    LazyLock::new(|| unbounded());

/// 获取日志发送端
fn log_sender() -> &'static Sender<LogEntry> {
    &LOG_CHANNEL.0
}

/// 获取日志接收端
fn log_receiver() -> &'static Receiver<LogEntry> {
    &LOG_CHANNEL.1
}

// ============================================================================
// 日志器实现
// ============================================================================

/// egui_kit 日志器
pub struct KitLogger;

impl log::Log for KitLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let entry = LogEntry::new(record.level(), format!("{}", record.args()));
            // 发送到通道 (非阻塞)
            let _ = log_sender().try_send(entry);
        }
    }

    fn flush(&self) {}
}

static KIT_LOGGER: KitLogger = KitLogger;

/// 初始化日志系统
///
/// # Example
/// ```no_run
/// use egui_kit::logger::init_logger;
/// init_logger().expect("Failed to init logger");
/// ```
pub fn init_logger() -> Result<(), log::SetLoggerError> {
    log::set_logger(&KIT_LOGGER)?;
    log::set_max_level(log::LevelFilter::Info);
    Ok(())
}

/// 设置日志级别
pub fn set_log_level(level: log::LevelFilter) {
    log::set_max_level(level);
}

// ============================================================================
// egui Memory 中的日志缓存
// ============================================================================

/// 日志缓存状态 (存储在 egui memory 中)
#[derive(Clone)]
struct LogCache {
    entries: VecDeque<LogEntry>,
    max_entries: usize,
}

impl Default for LogCache {
    fn default() -> Self {
        Self {
            entries: VecDeque::with_capacity(1000),
            max_entries: 1000,
        }
    }
}

impl LogCache {
    fn with_capacity(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
        }
    }

    /// 从通道接收新日志
    fn drain_channel(&mut self) {
        while let Ok(entry) = log_receiver().try_recv() {
            if self.entries.len() >= self.max_entries {
                self.entries.pop_front();
            }
            self.entries.push_back(entry);
        }
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter()
    }

    /// 获取所有日志的文本 (用于复制)
    fn to_text(&self) -> String {
        self.entries
            .iter()
            .map(|e| e.full_line())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// ============================================================================
// 日志面板 UI 组件
// ============================================================================

/// 日志面板配置
pub struct LogPanel {
    /// 唯一标识符
    id: Id,
    /// 最大日志条目数
    max_entries: usize,
    /// 字体大小
    font_size: f32,
    /// 行高
    line_height: f32,
    /// 是否显示工具栏
    show_toolbar: bool,
    /// 是否自动滚动到底部
    auto_scroll: bool,
}

impl Default for LogPanel {
    fn default() -> Self {
        Self {
            id: Id::new("egui_kit_log_panel"),
            max_entries: 1000,
            font_size: 13.0,
            line_height: 19.0,
            show_toolbar: true,
            auto_scroll: true,
        }
    }
}

impl LogPanel {
    /// 创建新的日志面板
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置唯一标识符 (用于区分多个日志面板)
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.id = Id::new(id);
        self
    }

    /// 设置最大日志条目数
    pub fn max_entries(mut self, max: usize) -> Self {
        self.max_entries = max;
        self
    }

    /// 设置字体大小
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// 设置行高
    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = height;
        self
    }

    /// 是否显示工具栏
    pub fn show_toolbar(mut self, show: bool) -> Self {
        self.show_toolbar = show;
        self
    }

    /// 是否自动滚动到底部
    pub fn auto_scroll(mut self, auto: bool) -> Self {
        self.auto_scroll = auto;
        self
    }

    /// 显示日志面板
    pub fn show(self, ui: &mut Ui) -> LogPanelResponse {
        let cache_id = self.id.with("cache");

        // 获取或创建日志缓存
        let mut cache = ui
            .ctx()
            .memory_mut(|mem| {
                mem.data
                    .get_temp_mut_or_insert_with(cache_id, || LogCache::with_capacity(self.max_entries))
                    .clone()
            });

        // 从通道接收新日志
        cache.drain_channel();

        let mut response = LogPanelResponse {
            cleared: false,
            copied: false,
            log_count: cache.len(),
        };

        let mut scroll_top = false;
        let mut scroll_bottom = false;

        // 工具栏
        if self.show_toolbar {
            ui.horizontal(|ui| {
                // 清空按钮
                if ui.button("🗑 清空").clicked() {
                    cache.clear();
                    response.cleared = true;
                }

                ui.add_space(10.0);

                // 复制按钮
                if ui
                    .add_enabled(!cache.is_empty(), egui::Button::new("📋 复制"))
                    .clicked()
                {
                    ui.ctx().copy_text(cache.to_text());
                    response.copied = true;
                }

                ui.add_space(10.0);

                // 滚动按钮
                if ui.button("⬆ 顶部").clicked() {
                    scroll_top = true;
                }
                if ui.button("⬇ 底部").clicked() {
                    scroll_bottom = true;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(format!("共 {} 条", cache.len())).color(Color32::GRAY));
                });
            });

            ui.separator();
            ui.add_space(5.0);
        }

        // 日志内容区域
        let mut font_id = egui::FontSelection::Default.resolve(ui.style());
        font_id.size = self.font_size;

        let base_text_format = egui::text::TextFormat {
            font_id,
            color: Color32::from_rgb(230, 230, 230),
            line_height: Some(self.line_height),
            ..Default::default()
        };

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(self.auto_scroll)
            .show(ui, |ui| {
                if scroll_top {
                    ui.scroll_to_cursor(Some(egui::Align::TOP));
                }
                if scroll_bottom {
                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                }

                if cache.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(RichText::new("暂无日志").color(Color32::GRAY).size(14.0));
                    });
                } else {
                    // 使用 LayoutJob 批量渲染日志 (性能优化)
                    let mut job = egui::text::LayoutJob::default();
                    job.wrap.break_anywhere = true;

                    for entry in cache.iter() {
                        let text = format!("{} {}\n", entry.timestamp, entry.message);
                        let mut text_format = base_text_format.clone();
                        text_format.color = entry.level_color();
                        job.append(&text, 0.0, text_format);
                    }

                    ui.label(job);
                }
            });

        // 保存缓存回 memory
        ui.ctx().memory_mut(|mem| {
            mem.data.insert_temp(cache_id, cache);
        });

        response
    }
}

/// 日志面板响应
pub struct LogPanelResponse {
    /// 是否点击了清空按钮
    pub cleared: bool,
    /// 是否点击了复制按钮
    pub copied: bool,
    /// 当前日志数量
    pub log_count: usize,
}

// ============================================================================
// 日志窗口 (独立浮动窗口)
// ============================================================================

/// 日志窗口
pub struct LogWindow {
    /// 窗口标题
    title: String,
    /// 是否打开
    open: bool,
    /// 窗口大小
    default_size: [f32; 2],
    /// 日志面板配置
    panel: LogPanel,
}

impl Default for LogWindow {
    fn default() -> Self {
        Self {
            title: "日志查看器".to_string(),
            open: false,
            default_size: [800.0, 600.0],
            panel: LogPanel::new(),
        }
    }
}

impl LogWindow {
    /// 创建新的日志窗口
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置窗口标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// 设置默认大小
    pub fn default_size(mut self, size: [f32; 2]) -> Self {
        self.default_size = size;
        self
    }

    /// 设置最大日志条目数
    pub fn max_entries(mut self, max: usize) -> Self {
        self.panel = self.panel.max_entries(max);
        self
    }

    /// 打开窗口
    pub fn open(&mut self) {
        self.open = true;
    }

    /// 关闭窗口
    pub fn close(&mut self) {
        self.open = false;
    }

    /// 切换窗口状态
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    /// 窗口是否打开
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// 显示日志窗口
    pub fn show(&mut self, ctx: &egui::Context) -> Option<LogPanelResponse> {
        if !self.open {
            return None;
        }

        let mut response = None;

        egui::Window::new(&self.title)
            .open(&mut self.open)
            .default_size(self.default_size)
            .resizable(true)
            .show(ctx, |ui| {
                response = Some(self.panel.clone().show(ui));
            });

        response
    }
}

impl Clone for LogPanel {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            max_entries: self.max_entries,
            font_size: self.font_size,
            line_height: self.line_height,
            show_toolbar: self.show_toolbar,
            auto_scroll: self.auto_scroll,
        }
    }
}

// ============================================================================
// 便捷函数
// ============================================================================

/// 获取当前日志数量 (从全局通道, 不需要 egui Context)
pub fn pending_log_count() -> usize {
    log_receiver().len()
}

/// 清空待处理的日志 (丢弃通道中的所有日志)
pub fn clear_pending_logs() {
    while log_receiver().try_recv().is_ok() {}
}
