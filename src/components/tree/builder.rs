use crate::list_item::{CustomContent, LabelContent, ListItem};
use crate::UiExt;
use egui::{Align2, FontId, Response, Sense, Ui, WidgetText};

use super::{TreeAction, TreeConfig, TreeNodeId, TreeState};

/// 单个节点的可选配置.
#[derive(Clone, Copy, Debug, Default)]
pub struct NodeOptions {
    pub icon: Option<&'static str>,
    pub checkbox: bool,
    pub default_open: bool,
    /// Allow drag source behavior for this row (DnD is handled by the caller via `Response`).
    pub draggable: bool,
    /// Override the internal egui id used for the collapsing state.
    ///
    /// This is useful when your `NodeId` is not globally unique but your UI path is.
    pub ui_id: Option<egui::Id>,
    /// Render text in a subdued (dimmed) style.
    pub subdued: bool,
    /// Override the row background (useful for highlighting specific node types).
    pub force_background: Option<egui::Color32>,
}

pub struct TreeBuilder<'ui, 'a, NodeId: TreeNodeId> {
    ui: &'ui mut Ui,
    tree_id: egui::Id,
    config: &'a TreeConfig,
    state: &'a mut TreeState<NodeId>,
    actions: &'a mut Vec<TreeAction<NodeId>>,
    all_nodes: &'a mut Vec<NodeId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RowKind {
    Leaf,
    Dir,
}

impl<'ui, 'a, NodeId: TreeNodeId> TreeBuilder<'ui, 'a, NodeId> {
    pub(super) fn new(
        ui: &'ui mut Ui,
        tree_id: egui::Id,
        config: &'a TreeConfig,
        state: &'a mut TreeState<NodeId>,
        actions: &'a mut Vec<TreeAction<NodeId>>,
        all_nodes: &'a mut Vec<NodeId>,
    ) -> Self {
        Self {
            ui,
            tree_id,
            config,
            state,
            actions,
            all_nodes,
        }
    }

    pub fn ui_mut(&mut self) -> &mut Ui {
        self.ui
    }

    fn base_item(&self, is_selected: bool) -> ListItem {
        let mut item = ListItem::new().selected(is_selected);
        if let Some(height) = self.config.row_height {
            item = item.with_height(height);
        }
        item
    }

    pub fn leaf(&mut self, id: NodeId, label: impl Into<WidgetText>, opts: NodeOptions) {
        let _ = self.render_leaf_response(id, label.into(), opts);
    }

    /// Render a leaf node and return the row response.
    ///
    /// Useful when the caller needs access to the `Response` for e.g. drag-and-drop.
    pub fn leaf_with_response(&mut self, id: NodeId, label: impl Into<WidgetText>, opts: NodeOptions) -> Response {
        self.render_leaf_response(id, label.into(), opts)
    }

    pub fn dir(&mut self, id: NodeId, label: impl Into<WidgetText>, opts: NodeOptions, children: impl FnOnce(&mut TreeBuilder<'_, 'a, NodeId>)) {
        let _ = self.render_dir_response(id, label.into(), opts, children);
    }

    /// Render a container node and return the collapsing response.
    ///
    /// Useful when the caller needs access to `item_response` + `body_response` for e.g. drop zones.
    pub fn dir_with_response(
        &mut self,
        id: NodeId,
        label: impl Into<WidgetText>,
        opts: NodeOptions,
        children: impl FnOnce(&mut TreeBuilder<'_, 'a, NodeId>),
    ) -> crate::list_item::ShowCollapsingResponse<()> {
        self.render_dir_response(id, label.into(), opts, children)
    }

    fn render_leaf_response(&mut self, id: NodeId, label: WidgetText, opts: NodeOptions) -> Response {
        let is_selected = self.state.is_selected(&id);
        self.all_nodes.push(id.clone());

        let response = if opts.checkbox {
            self.show_checkbox_row(&id, is_selected, label, opts.icon)
        } else {
            let mut content = LabelContent::new(label).subdued(opts.subdued);
            if let Some(icon) = opts.icon {
                content = content.with_phosphor_icon(icon);
            }
            let mut item = self.base_item(is_selected).draggable(opts.draggable);
            if let Some(bg) = opts.force_background {
                item = item.force_background(bg);
            }
            item.show_hierarchical(self.ui, content)
        };

        self.handle_row_response(id, response.clone(), RowKind::Leaf);
        response
    }

    fn render_dir_response(
        &mut self,
        id: NodeId,
        label: WidgetText,
        opts: NodeOptions,
        children: impl FnOnce(&mut TreeBuilder<'_, 'a, NodeId>),
    ) -> crate::list_item::ShowCollapsingResponse<()> {
        let is_selected = self.state.is_selected(&id);
        let is_expanded = if self.state.is_interacted(&id) {
            self.state.is_expanded(&id)
        } else {
            opts.default_open || self.config.default_expanded
        };
        self.all_nodes.push(id.clone());

        // 关键: 子节点必须在 show_hierarchical_with_children 的闭包内渲染,
        // 才能复用 list_item 的 show_body_indented 缩进与竖线.
        let egui_id = opts.ui_id.unwrap_or_else(|| self.tree_id.with(("tree_node", &id)));
        let old_is_open = is_expanded;

        // 由于 closure 需要可变引用, 先把借用拆开避免捕获 self.
        let tree_id = self.tree_id;
        let config = self.config;
        let state: *mut TreeState<NodeId> = self.state;
        let actions: *mut Vec<TreeAction<NodeId>> = self.actions;
        let all_nodes: *mut Vec<NodeId> = self.all_nodes;

        let collapsing = if opts.checkbox {
            // 目录也允许 checkbox
            let checked_now = self.state.is_checked(&id);
            let change = std::rc::Rc::new(std::cell::Cell::new(None));
            let content = self.make_checkbox_content(checked_now, label.clone(), opts.icon, change.clone());

            let mut item = self
                .base_item(is_selected)
                .draggable(opts.draggable)
                .click_to_toggle(config.click_to_toggle);
            if let Some(bg) = opts.force_background {
                item = item.force_background(bg);
            }

            let response = item.show_hierarchical_with_children(self.ui, egui_id, is_expanded, content, |ui| unsafe {
                    // SAFETY: TreeView 在单线程 UI 帧内调用, 这里仅在同一调用栈内使用.
                    let state = &mut *state;
                    let actions = &mut *actions;
                    let all_nodes = &mut *all_nodes;
                    let mut builder = TreeBuilder::new(ui, tree_id, config, state, actions, all_nodes);
                    children(&mut builder);
                });

            if let Some(new_value) = change.get() {
                self.state.set_checked(id.clone(), new_value);
                self.actions
                    .push(TreeAction::CheckedChanged { id: id.clone(), checked: new_value });
            }

            response
        } else {
            let mut content = LabelContent::new(label.clone()).subdued(opts.subdued);
            if let Some(icon) = opts.icon {
                content = content.with_phosphor_icon(icon);
            }

            let mut item = self
                .base_item(is_selected)
                .draggable(opts.draggable)
                .click_to_toggle(config.click_to_toggle);
            if let Some(bg) = opts.force_background {
                item = item.force_background(bg);
            }

            item.show_hierarchical_with_children(self.ui, egui_id, is_expanded, content, |ui| unsafe {
                    // SAFETY: TreeView 在单线程 UI 帧内调用, 这里仅在同一调用栈内使用.
                    let state = &mut *state;
                    let actions = &mut *actions;
                    let all_nodes = &mut *all_nodes;
                    let mut builder = TreeBuilder::new(ui, tree_id, config, state, actions, all_nodes);
                    children(&mut builder);
                })
        };

        let new_is_open = collapsing.openness > 0.5;
        if new_is_open != old_is_open {
            if new_is_open {
                self.state.expand(id.clone());
            } else {
                self.state.collapse(&id);
            }
            self.actions.push(TreeAction::Toggled { id: id.clone(), expanded: new_is_open });
        }

        self.handle_row_response(id, collapsing.item_response.clone(), RowKind::Dir);

        collapsing
    }

    fn make_checkbox_content(
        &self,
        checked_now: bool,
        label: WidgetText,
        icon: Option<&'static str>,
        change: std::rc::Rc<std::cell::Cell<Option<bool>>>,
    ) -> CustomContent<'static> {
        // NOTE: 这里返回 'static 仅因为 closure 捕获的都是 owned 值.
        // CustomContent 的 closure 会在当前帧内被调用.
        CustomContent::new(move |ui, _ctx| {
            let tokens = ui.tokens();
            let mut value = checked_now;
            let resp = ui.checkbox(&mut value, "");
            if resp.changed() {
                change.set(Some(value));
            }

            ui.add_space(tokens.text_to_icon_padding());

            if let Some(icon) = icon {
                let (icon_rect, _) = ui.allocate_exact_size(tokens.small_icon_size, Sense::hover());
                ui.painter().text(
                    icon_rect.center(),
                    Align2::CENTER_CENTER,
                    icon,
                    FontId::proportional(icon_rect.height() * 0.8),
                    ui.visuals().text_color(),
                );
                ui.add_space(tokens.text_to_icon_padding());
            }

            ui.label(label);
        })
    }

    fn show_checkbox_row(&mut self, id: &NodeId, is_selected: bool, label: WidgetText, icon: Option<&'static str>) -> Response {
        let checked_now = self.state.is_checked(id);

        let change = std::rc::Rc::new(std::cell::Cell::new(None));

        let content = self.make_checkbox_content(checked_now, label, icon, change.clone());
        let response = self.base_item(is_selected).show_hierarchical(self.ui, content);
        if let Some(new_value) = change.get() {
            self.state.set_checked(id.clone(), new_value);
            self.actions.push(TreeAction::CheckedChanged { id: id.clone(), checked: new_value });
        }
        response
    }

    fn handle_row_response(&mut self, id: NodeId, response: Response, kind: RowKind) {
        let modifiers = self.ui.input(|i| i.modifiers);

        if response.double_clicked() {
            // 目录节点的双击由 ListItem 负责展开/折叠.
            if kind == RowKind::Leaf {
                self.actions.push(TreeAction::Activated(id.clone()));
            }
        } else if response.clicked() {
            self.handle_click(id.clone(), modifiers);
        }

        if response.secondary_clicked() {
            let pos = self.ui.input(|i| i.pointer.interact_pos().unwrap_or_default());
            self.actions.push(TreeAction::ContextMenuRequested { id, position: pos });
        }
    }

    fn handle_click(&mut self, id: NodeId, modifiers: egui::Modifiers) {
        let prev_selected = self.state.selected().to_vec();

        if self.config.allow_multi_select {
            if modifiers.matches_exact(self.config.range_select_modifier) {
                if let Some(pivot) = prev_selected.first() {
                    let range = self.collect_range(pivot, &id);
                    self.state.set_selected(range);
                } else {
                    self.state.set_selected(vec![id]);
                }
            } else if modifiers.matches_exact(self.config.toggle_select_modifier) {
                let mut selected = prev_selected.clone();
                if let Some(pos) = selected.iter().position(|x| x == &id) {
                    selected.remove(pos);
                } else {
                    selected.push(id);
                }
                self.state.set_selected(selected);
            } else {
                self.state.set_selected(vec![id]);
            }
        } else {
            self.state.set_selected(vec![id]);
        }

        if self.state.selected() != prev_selected.as_slice() {
            self.actions.push(TreeAction::SelectionChanged(self.state.selected().to_vec()));
        }
    }

    fn collect_range(&self, from: &NodeId, to: &NodeId) -> Vec<NodeId> {
        let from_idx = self.all_nodes.iter().position(|x| x == from);
        let to_idx = self.all_nodes.iter().position(|x| x == to);

        match (from_idx, to_idx) {
            (Some(f), Some(t)) => {
                let start = f.min(t);
                let end = f.max(t);
                self.all_nodes[start..=end].to_vec()
            }
            _ => vec![to.clone()],
        }
    }
}
