use eframe::egui;
use egui_kit::{TreeAction, TreeView};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum NodeId {
    Dept(u32),
    User(u32),
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Tree Prefix/Suffix Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    selected_nodes: Vec<NodeId>,
    dept_selections: std::collections::HashMap<u32, SelectionState>,
    user_selections: std::collections::HashMap<u32, SelectionState>,
}

#[derive(Clone, Copy, PartialEq)]
enum SelectionState {
    Unchecked,
    Checked,
    Partial,
}

impl MyApp {
    fn render_tree(&mut self, ui: &mut egui::Ui) {
        let mut tree_view = TreeView::new("dept_tree");
        let actions = tree_view.show(ui, |builder| {
            // 部门 1 (Marketing)
            builder
                .dir(NodeId::Dept(1), |ui| {
                    ui.label("Marketing");
                })
                .prefix(|ui| {
                    let state = self
                        .dept_selections
                        .get(&1)
                        .copied()
                        .unwrap_or(SelectionState::Unchecked);
                    let icon = match state {
                        SelectionState::Unchecked => "☐",
                        SelectionState::Checked => "☑",
                        SelectionState::Partial => "⊟",
                    };
                    ui.label(icon);
                })
                .suffix(|ui| {
                    ui.label(egui::RichText::new("3 users").small().weak());
                })
                .children(|builder| {
                    // 用户 101
                    builder
                        .leaf(NodeId::User(101), |ui| {
                            ui.label("Alice");
                        })
                        .prefix(|ui| {
                            let checked = self.user_selections.get(&101).copied()
                                == Some(SelectionState::Checked);
                            let icon = if checked { "☑" } else { "☐" };
                            if ui.selectable_label(false, icon).clicked() {
                                self.user_selections.insert(
                                    101,
                                    if checked {
                                        SelectionState::Unchecked
                                    } else {
                                        SelectionState::Checked
                                    },
                                );
                            }
                        })
                        .suffix(|ui| {
                            ui.label(
                                egui::RichText::new("已分享")
                                    .color(egui::Color32::GREEN)
                                    .small(),
                            );
                        });

                    // 用户 102
                    builder
                        .leaf(NodeId::User(102), |ui| {
                            ui.label("Bob");
                        })
                        .prefix(|ui| {
                            let checked = self.user_selections.get(&102).copied()
                                == Some(SelectionState::Checked);
                            let icon = if checked { "☑" } else { "☐" };
                            if ui.selectable_label(false, icon).clicked() {
                                self.user_selections.insert(
                                    102,
                                    if checked {
                                        SelectionState::Unchecked
                                    } else {
                                        SelectionState::Checked
                                    },
                                );
                            }
                        })
                        .suffix(|ui| {
                            ui.label(
                                egui::RichText::new("待撤销")
                                    .color(egui::Color32::YELLOW)
                                    .small(),
                            );
                        });

                    // 用户 103
                    builder
                        .leaf(NodeId::User(103), |ui| {
                            ui.label("Charlie");
                        })
                        .prefix(|ui| {
                            let checked = self.user_selections.get(&103).copied()
                                == Some(SelectionState::Checked);
                            let icon = if checked { "☑" } else { "☐" };
                            if ui.selectable_label(false, icon).clicked() {
                                self.user_selections.insert(
                                    103,
                                    if checked {
                                        SelectionState::Unchecked
                                    } else {
                                        SelectionState::Checked
                                    },
                                );
                            }
                        });
                });

            // 部门 2 (Engineering)
            builder
                .dir(NodeId::Dept(2), |ui| {
                    ui.label("Engineering");
                })
                .default_open(true)
                .prefix(|ui| {
                    let state = self
                        .dept_selections
                        .get(&2)
                        .copied()
                        .unwrap_or(SelectionState::Unchecked);
                    let icon = match state {
                        SelectionState::Unchecked => "☐",
                        SelectionState::Checked => "☑",
                        SelectionState::Partial => "⊟",
                    };
                    ui.label(icon);
                })
                .suffix(|ui| {
                    ui.label(egui::RichText::new("2 users").small().weak());
                })
                .children(|builder| {
                    builder
                        .leaf(NodeId::User(201), |ui| {
                            ui.label("Dave");
                        })
                        .prefix(|ui| {
                            let checked = self.user_selections.get(&201).copied()
                                == Some(SelectionState::Checked);
                            let icon = if checked { "☑" } else { "☐" };
                            if ui.selectable_label(false, icon).clicked() {
                                self.user_selections.insert(
                                    201,
                                    if checked {
                                        SelectionState::Unchecked
                                    } else {
                                        SelectionState::Checked
                                    },
                                );
                            }
                        })
                        .suffix(|ui| {
                            ui.label(
                                egui::RichText::new("已分享")
                                    .color(egui::Color32::GREEN)
                                    .small(),
                            );
                        });

                    builder
                        .leaf(NodeId::User(202), |ui| {
                            ui.label("Eve");
                        })
                        .prefix(|ui| {
                            let checked = self.user_selections.get(&202).copied()
                                == Some(SelectionState::Checked);
                            let icon = if checked { "☑" } else { "☐" };
                            if ui.selectable_label(false, icon).clicked() {
                                self.user_selections.insert(
                                    202,
                                    if checked {
                                        SelectionState::Unchecked
                                    } else {
                                        SelectionState::Checked
                                    },
                                );
                            }
                        })
                        .suffix(|ui| {
                            ui.label(egui::RichText::new("").small());
                        });
                });
        });

        for action in actions {
            match action {
                TreeAction::SelectionChanged(nodes) => {
                    self.selected_nodes = nodes;
                }
                TreeAction::Activated { nodes, .. } => {
                    println!("Double-clicked: {nodes:?}");
                }
                _ => {}
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tree with Prefix/Suffix");
            ui.separator();

            ui.label("演示:");
            ui.label("• Prefix: 自定义选择框");
            ui.label("• Suffix: 状态标签");
            ui.label("• 点击选择框切换状态");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_tree(ui);
            });

            ui.separator();
            ui.label(format!("Tree 选中: {:?}", self.selected_nodes));

            let checked_users: Vec<_> = self
                .user_selections
                .iter()
                .filter(|(_, s)| **s == SelectionState::Checked)
                .map(|(id, _)| *id)
                .collect();
            ui.label(format!("选择框选中用户: {:?}", checked_users));
        });
    }
}
