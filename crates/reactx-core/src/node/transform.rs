//! `<Transform>` output and user-id lookups on the node tree.

use super::*;

impl NodeTree {
    /// Find a node by the user-visible `id` attribute.
    pub fn find_by_user_id(&self, wanted: &str) -> Option<NodeId> {
        self.nodes
            .values()
            .find(|node| node.attrs.custom.get("id").is_some_and(|id| id == wanted))
            .map(|node| node.id)
    }

    /// Recompute `<Transform>` output for every marked node, children first so
    /// nested transforms compose.
    pub fn apply_transforms(&mut self) {
        self.apply_transforms_from(self.root);
    }

    fn apply_transforms_from(&mut self, id: NodeId) {
        let children = match self.nodes.get(&id) {
            Some(node) => node.children.clone(),
            None => return,
        };
        for child in children {
            self.apply_transforms_from(child);
        }
        let Some(key) = self
            .nodes
            .get(&id)
            .and_then(|node| node.attrs.custom.get("data_transform"))
            .cloned()
        else {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.text_override = None;
            }
            return;
        };
        let Some(transform) = crate::transform::lookup(&key) else {
            return;
        };
        // Drop the previous frame's output before collecting child text, or a
        // stale override would feed back into its own input.
        if let Some(node) = self.nodes.get_mut(&id) {
            node.text_override = None;
        }
        let text = self.collect_text(id);
        if let Some(node) = self.nodes.get_mut(&id) {
            node.text_override = Some(transform(&text));
        }
    }
}
