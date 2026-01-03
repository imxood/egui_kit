//! Dialog 对话框组件示例
//!
//! 演示两种对话框类型:
//! - 普通文本对话框 (不可取消)
//! - 可取消对话框 (带回调和用户数据)
//!
//! 运行方式:
//! cargo run -p egui_kit --example dialog_demo

use eframe::egui;
use egui_kit::{dialog::Dialog, setup_theme, ThemePreset};
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_title("Dialog Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Dialog Demo",
        native_options,
        Box::new(|cc| {
            setup_theme(&cc.egui_ctx, ThemePreset::Dark);
            Ok(Box::new(DemoApp::default()))
        }),
    )
}

#[derive(Default)]
struct DemoApp {
    cancel_count: Arc<AtomicU32>,
    last_cancelled_task: Option<String>,
    log_messages: Vec<String>,
}

impl DemoApp {
    fn log(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        println!("{}", msg);
        self.log_messages.push(msg);
        // 保留最近 10 条
        if self.log_messages.len() > 10 {
            self.log_messages.remove(0);
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dialog Demo");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Cancel count:");
                ui.label(format!("{}", self.cancel_count.load(Ordering::Relaxed)));
            });

            if let Some(task) = &self.last_cancelled_task {
                ui.label(format!("Last cancelled: {}", task));
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            // ==================== 普通文本对话框 ====================
            ui.label("1. Text Dialog (not cancellable)");
            ui.horizontal(|ui| {
                if ui.button("Show Loading").clicked() {
                    Dialog::show_text(ctx, "loading", "Loading data...");
                    self.log("Showing text dialog: loading");
                }

                if ui.button("Show Processing").clicked() {
                    Dialog::show_text(ctx, "processing", "Processing, please wait...");
                    self.log("Showing text dialog: processing");
                }

                if ui.button("Show Large Dialog").clicked() {
                    Dialog::show_text_with_size(
                        ctx,
                        "large",
                        "This is a larger dialog\nwith custom size",
                        egui::vec2(400.0, 250.0),
                    );
                    self.log("Showing large text dialog: 400x250");
                }

                if ui.button("Close All Text").clicked() {
                    Dialog::close(ctx, "loading");
                    Dialog::close(ctx, "processing");
                    Dialog::close(ctx, "large");
                    self.log("Closed text dialogs");
                }
            });

            ui.add_space(20.0);

            // ==================== 可取消对话框 ====================
            ui.label("2. Cancellable Dialog (with callback)");
            ui.horizontal(|ui| {
                if ui.button("Show Download").clicked() {
                    let cancel_count = self.cancel_count.clone();
                    Dialog::show_cancellable(
                        ctx,
                        "download",
                        "Downloading file...",
                        "Cancel",
                        "download_task_001".to_string(),
                        move |task_id| {
                            cancel_count.fetch_add(1, Ordering::Relaxed);
                            println!("Cancelled task: {}", task_id);
                        },
                    );
                    self.log("Showing cancellable dialog: download");
                }

                if ui.button("Show Upload").clicked() {
                    let cancel_count = self.cancel_count.clone();
                    Dialog::show_cancellable(
                        ctx,
                        "upload",
                        "Uploading data...",
                        "Stop",
                        42u64, // user data: task id
                        move |task_id| {
                            cancel_count.fetch_add(1, Ordering::Relaxed);
                            println!("Cancelled upload task: {}", task_id);
                        },
                    );
                    self.log("Showing cancellable dialog: upload");
                }
            });

            ui.add_space(10.0);

            // 带复杂数据的对话框
            ui.label("3. Cancellable Dialog (with complex data)");
            ui.horizontal(|ui| {
                if ui.button("Show Run Test").clicked() {
                    #[derive(Clone, Debug)]
                    struct RunContext {
                        project_id: String,
                        way_ids: Vec<u32>,
                    }

                    let run_ctx = RunContext {
                        project_id: "PROJECT_001".to_string(),
                        way_ids: vec![1, 2, 3, 4],
                    };

                    let cancel_count = self.cancel_count.clone();
                    Dialog::show_cancellable(
                        ctx,
                        "run_test",
                        "Running test...",
                        "Stop Test",
                        run_ctx,
                        move |ctx| {
                            cancel_count.fetch_add(1, Ordering::Relaxed);
                            println!(
                                "Stopped test - project: {}, ways: {:?}",
                                ctx.project_id, ctx.way_ids
                            );
                        },
                    );
                    self.log("Showing cancellable dialog: run_test");
                }

                if ui.button("Show Wide Dialog").clicked() {
                    let cancel_count = self.cancel_count.clone();
                    Dialog::show_cancellable_with_size(
                        ctx,
                        "wide",
                        "This is a wide dialog with custom size\nYou can customize width and height",
                        "Cancel",
                        egui::vec2(500.0, 200.0),
                        "wide_task".to_string(),
                        move |task| {
                            cancel_count.fetch_add(1, Ordering::Relaxed);
                            println!("Cancelled wide dialog task: {}", task);
                        },
                    );
                    self.log("Showing wide cancellable dialog: 500x200");
                }
            });

            ui.add_space(20.0);

            // ==================== 控制按钮 ====================
            ui.separator();
            ui.add_space(10.0);
            ui.label("Controls:");
            ui.horizontal(|ui| {
                if ui.button("Close All").clicked() {
                    Dialog::close_all(ctx);
                    self.log("Closed all dialogs");
                }

                ui.label(format!(
                    "Active dialogs: {}",
                    if Dialog::is_showing(ctx) { "Yes" } else { "No" }
                ));
            });

            ui.add_space(20.0);

            // ==================== 日志 ====================
            ui.separator();
            ui.label("Log:");
            egui::ScrollArea::vertical()
                .max_height(100.0)
                .show(ui, |ui| {
                    for msg in &self.log_messages {
                        ui.label(msg);
                    }
                });
        });

        // 渲染对话框 (必须在末尾调用)
        Dialog::render(ctx);
    }
}
