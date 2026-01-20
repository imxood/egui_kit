//!
//! TreeView 示例: 点击展开, 多选, 复选框.
//!
//! 运行方式:
//! RUSTFLAGS="-A warnings" cargo check -q --message-format short -p egui_kit --example tree_basic

use eframe::egui;
use egui_kit::{icons, setup_theme, ThemeName, TreeAction, TreeView, NodeOptions};
use egui_kit::utils::font::FontManager;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([820.0, 620.0])
            .with_title("Tree Component Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Tree Component Demo",
        native_options,
        Box::new(|cc| {
            setup_theme(&cc.egui_ctx, ThemeName::ModernDark);
            let font_manager = FontManager::new(&cc.egui_ctx).ok();
            Ok(Box::new(DemoApp {
                allow_multi_select: false,
                log_messages: Vec::new(),
                font_manager,
            }))
        }),
    )
}

#[derive(Default)]
struct DemoApp {
    allow_multi_select: bool,
    log_messages: Vec<String>,
    #[allow(dead_code)]
    font_manager: Option<FontManager>,
}

impl DemoApp {
    fn log(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        println!("{}", msg);
        self.log_messages.push(msg);
        if self.log_messages.len() > 15 {
            self.log_messages.remove(0);
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        setup_theme(ctx, ThemeName::ModernDark);
        egui::SidePanel::left("control_panel")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("Tree Demo");
                ui.add_space(10.0);

                ui.label("Configuration:");
                ui.checkbox(&mut self.allow_multi_select, "Allow Multi-Select");
                ui.add_space(10.0);

                ui.separator();
                ui.add_space(10.0);

                ui.label("Event Log:");
                egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                    for msg in &self.log_messages {
                        ui.label(msg);
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Explorer");
            ui.add_space(8.0);

            let actions = TreeView::new("demo_tree")
                .allow_multi_select(self.allow_multi_select)
                .click_to_toggle(false)
                .show_indent_guides(true)
                .show(ui, |builder| {
                    builder.dir(
                        "root",
                        "Project Root",
                        NodeOptions {
                            icon: Some(icons::FOLDER),
                            checkbox: false,
                            default_open: true,
                            draggable: false,
                            ui_id: None,
                            subdued: false,
                            force_background: None,
                        },
                        |builder| {
                            builder.dir(
                                "src",
                                "src",
                                NodeOptions {
                                    icon: Some(icons::FOLDER),
                                    checkbox: false,
                                    default_open: true,
                                    draggable: false,
                                    ui_id: None,
                                    subdued: false,
                                    force_background: None,
                                },
                                |builder| {
                                    builder.leaf(
                                        "main",
                                        "main.rs",
                                        NodeOptions {
                                            icon: Some(icons::FILE_DOC),
                                            checkbox: false,
                                            default_open: false,
                                            draggable: false,
                                            ui_id: None,
                                            subdued: false,
                                            force_background: None,
                                        },
                                    );
                                    builder.leaf(
                                        "lib",
                                        "lib.rs",
                                        NodeOptions {
                                            icon: Some(icons::FILE_DOC),
                                            checkbox: false,
                                            default_open: false,
                                            draggable: false,
                                            ui_id: None,
                                            subdued: false,
                                            force_background: None,
                                        },
                                    );
                                },
                            );

                            builder.dir(
                                "assets",
                                "assets",
                                NodeOptions {
                                    icon: Some(icons::FOLDER),
                                    checkbox: false,
                                    default_open: false,
                                    draggable: false,
                                    ui_id: None,
                                    subdued: false,
                                    force_background: None,
                                },
                                |builder| {
                                    builder.leaf(
                                        "logo",
                                        "logo.png",
                                        NodeOptions {
                                            icon: Some(icons::FILE_DOC),
                                            checkbox: false,
                                            default_open: false,
                                            draggable: false,
                                            ui_id: None,
                                            subdued: false,
                                            force_background: None,
                                        },
                                    );
                                },
                            );
                        },
                    );
                });

            for action in actions {
                match action {
                    TreeAction::SelectionChanged(ids) => {
                        self.log(format!("Selection: {:?}", ids));
                    }
                    TreeAction::Activated(id) => {
                        self.log(format!("Activated: {}", id));
                    }
                    TreeAction::Toggled { id, expanded } => {
                        self.log(format!("Toggled: {} -> {}", id, expanded));
                    }
                    TreeAction::CheckedChanged { id, checked } => {
                        self.log(format!("Checked: {} -> {}", id, checked));
                    }
                    TreeAction::ContextMenuRequested { id, .. } => {
                        self.log(format!("Context menu: {}", id));
                    }
                }
            }
        });
    }
}
