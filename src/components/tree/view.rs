use crate::list_item::list_item_scope;
use crate::UiExt;

use super::{TreeAction, TreeBuilder, TreeConfig, TreeNodeId, TreeState};

pub struct TreeView<NodeId: TreeNodeId> {
    id: egui::Id,
    config: TreeConfig,
    _phantom: std::marker::PhantomData<NodeId>,
}

impl<NodeId: TreeNodeId> TreeView<NodeId> {
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: egui::Id::new(id),
            config: TreeConfig::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn config(mut self, config: TreeConfig) -> Self {
        self.config = config;
        self
    }

    pub fn indent(mut self, indent: f32) -> Self {
        self.config.indent = Some(indent);
        self
    }

    pub fn allow_multi_select(mut self, allow: bool) -> Self {
        self.config.allow_multi_select = allow;
        self
    }

    pub fn click_to_toggle(mut self, enable: bool) -> Self {
        self.config.click_to_toggle = enable;
        self
    }

    pub fn show_indent_guides(mut self, show: bool) -> Self {
        self.config.show_indent_guides = show;
        self
    }

    pub fn default_expanded(mut self, expanded: bool) -> Self {
        self.config.default_expanded = expanded;
        self
    }

    pub fn show(&self, ui: &mut egui::Ui, build_tree: impl FnOnce(&mut TreeBuilder<NodeId>)) -> Vec<TreeAction<NodeId>> {
        let mut state = TreeState::load(ui.ctx(), self.id);
        let mut actions = Vec::new();
        let mut all_nodes = Vec::new();

        let prev_indent_guides = ui.style().visuals.indent_has_left_vline;
        if !self.config.show_indent_guides {
            ui.style_mut().visuals.indent_has_left_vline = false;
        }

        list_item_scope(ui, self.id, |ui| {
            let mut builder = TreeBuilder::new(ui, self.id, &self.config, &mut state, &mut actions, &mut all_nodes);
            build_tree(&mut builder);
        });

        ui.style_mut().visuals.indent_has_left_vline = prev_indent_guides;

        state.save(ui.ctx(), self.id);
        actions
    }

    pub fn state(&self, ctx: &egui::Context) -> TreeState<NodeId> {
        TreeState::load(ctx, self.id)
    }

    pub fn modify_state(&self, ctx: &egui::Context, modify: impl FnOnce(&mut TreeState<NodeId>)) {
        let mut state = TreeState::load(ctx, self.id);
        modify(&mut state);
        state.save(ctx, self.id);
    }
}
