use egui::Modifiers;

#[derive(Clone, Debug)]
pub struct TreeConfig {
    pub indent: Option<f32>,
    pub row_height: Option<f32>,
    pub allow_multi_select: bool,
    pub click_to_toggle: bool,
    pub show_indent_guides: bool,
    pub default_expanded: bool,
    pub range_select_modifier: Modifiers,
    pub toggle_select_modifier: Modifiers,
}

impl Default for TreeConfig {
    fn default() -> Self {
        Self {
            indent: None,
            row_height: None,
            allow_multi_select: false,
            click_to_toggle: false,
            show_indent_guides: true,
            default_expanded: false,
            range_select_modifier: Modifiers::SHIFT,
            toggle_select_modifier: Modifiers::COMMAND,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TreeAction<NodeId> {
    SelectionChanged(Vec<NodeId>),
    Activated(NodeId),
    Toggled { id: NodeId, expanded: bool },
    CheckedChanged { id: NodeId, checked: bool },
    ContextMenuRequested { id: NodeId, position: egui::Pos2 },
}

pub trait TreeNodeId: Clone + Eq + std::hash::Hash + Send + Sync + 'static {}

impl<T: Clone + Eq + std::hash::Hash + Send + Sync + 'static> TreeNodeId for T {}
