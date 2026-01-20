use super::TreeNodeId;
use std::collections::HashSet;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TreeState<NodeId: TreeNodeId> {
    selected: Vec<NodeId>,
    expanded: HashSet<NodeId>,
    checked: HashSet<NodeId>,
    interacted: HashSet<NodeId>,
}

impl<NodeId: TreeNodeId> Default for TreeState<NodeId> {
    fn default() -> Self {
        Self {
            selected: Vec::new(),
            expanded: HashSet::new(),
            checked: HashSet::new(),
            interacted: HashSet::new(),
        }
    }
}

impl<NodeId: TreeNodeId> TreeState<NodeId> {
    pub fn load(ctx: &egui::Context, id: egui::Id) -> Self {
        ctx.data_mut(|d| d.get_temp(id).unwrap_or_default())
    }

    pub fn save(&self, ctx: &egui::Context, id: egui::Id) {
        ctx.data_mut(|d| d.insert_temp(id, self.clone()));
    }

    pub fn selected(&self) -> &[NodeId] {
        &self.selected
    }

    pub fn expanded(&self) -> &HashSet<NodeId> {
        &self.expanded
    }

    pub fn checked(&self) -> &HashSet<NodeId> {
        &self.checked
    }

    pub fn is_selected(&self, id: &NodeId) -> bool {
        self.selected.contains(id)
    }

    pub fn set_selected(&mut self, selected: Vec<NodeId>) {
        self.selected = selected;
    }

    pub fn is_expanded(&self, id: &NodeId) -> bool {
        self.expanded.contains(id)
    }

    pub fn expand(&mut self, id: NodeId) {
        self.expanded.insert(id.clone());
        self.interacted.insert(id);
    }

    pub fn collapse(&mut self, id: &NodeId) {
        self.expanded.remove(id);
        self.interacted.insert(id.clone());
    }

    /// Expand all node ids in the given path.
    ///
    /// This is primarily used for "auto expand to selected" behavior.
    pub fn expand_path(&mut self, path: impl IntoIterator<Item = NodeId>) {
        for id in path {
            self.expand(id);
        }
    }

    pub fn sync_expanded(&mut self, desired: &HashSet<NodeId>) {
        // Collapse removed nodes first.
        let current: Vec<NodeId> = self.expanded.iter().cloned().collect();
        for id in current {
            if !desired.contains(&id) {
                self.collapse(&id);
            }
        }

        // Expand missing nodes.
        for id in desired.iter().cloned() {
            if !self.expanded.contains(&id) {
                self.expand(id);
            }
        }
    }

    pub fn is_interacted(&self, id: &NodeId) -> bool {
        self.interacted.contains(id)
    }

    pub fn is_checked(&self, id: &NodeId) -> bool {
        self.checked.contains(id)
    }

    pub fn set_checked(&mut self, id: NodeId, checked: bool) {
        if checked {
            self.checked.insert(id);
        } else {
            self.checked.remove(&id);
        }
    }
}
