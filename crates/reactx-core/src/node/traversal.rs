//! Read-only traversal and hit ordering for [`NodeTree`].

use super::*;

impl NodeTree {
    pub fn ancestors(&self, id: NodeId) -> Vec<NodeId> {
        let mut out = Vec::new();
        let mut current = self.nodes.get(&id).and_then(|node| node.parent);
        while let Some(parent) = current {
            out.push(parent);
            current = self.nodes.get(&parent).and_then(|node| node.parent);
        }
        out
    }

    /// Collect descendant text, in document order, without separators.
    pub fn collect_text(&self, id: NodeId) -> String {
        let mut out = String::new();
        self.collect_text_into(id, &mut out);
        out
    }

    fn collect_text_into(&self, id: NodeId, out: &mut String) {
        let Some(node) = self.nodes.get(&id) else {
            return;
        };
        if let Some(text) = &node.text {
            out.push_str(text);
        }
        for child in &node.children {
            self.collect_text_into(*child, out);
        }
    }

    pub fn focusable_ids_in_order(&self) -> Vec<NodeId> {
        let mut out = Vec::new();
        self.collect_focusable(self.root, &mut out);
        out
    }

    fn collect_focusable(&self, id: NodeId, out: &mut Vec<NodeId>) {
        let Some(node) = self.nodes.get(&id) else {
            return;
        };
        if node.is_placeholder() || node.is_hidden() {
            return;
        }
        if id != self.root && node.is_focusable() {
            out.push(id);
        }
        for child in &node.children {
            self.collect_focusable(*child, out);
        }
    }

    /// Iterate all visible nodes in document order.
    pub fn nodes_in_order(&self) -> Vec<NodeId> {
        let mut out = Vec::new();
        self.collect_nodes(self.root, &mut out);
        out
    }

    fn collect_nodes(&self, id: NodeId, out: &mut Vec<NodeId>) {
        let Some(node) = self.nodes.get(&id) else {
            return;
        };
        if node.is_placeholder() {
            return;
        }
        out.push(id);
        for child in &node.children {
            self.collect_nodes(*child, out);
        }
    }

    /// Iterate all visible nodes in paint order (siblings sorted by layer).
    pub fn paint_order(&self) -> Vec<NodeId> {
        let mut out = Vec::new();
        self.collect_paint_order(self.root, &mut out);
        out
    }

    fn collect_paint_order(&self, id: NodeId, out: &mut Vec<NodeId>) {
        let Some(node) = self.nodes.get(&id) else {
            return;
        };
        if node.is_placeholder() || node.is_hidden() {
            return;
        }
        out.push(id);
        for child in self.sorted_children(node) {
            self.collect_paint_order(child, out);
        }
    }

    /// Focusable nodes in paint order, matching what the pointer sees on top.
    pub fn focusable_ids_in_paint_order(&self) -> Vec<NodeId> {
        self.paint_order()
            .into_iter()
            .filter(|id| *id != self.root)
            .filter(|id| {
                self.nodes
                    .get(id)
                    .map(TuiNode::is_focusable)
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Children sorted by paint layer, preserving document order within a layer.
    pub fn sorted_children<'a>(&'a self, node: &'a TuiNode) -> Vec<NodeId> {
        let mut children = node.children.clone();
        children.sort_by_key(|id| {
            self.nodes
                .get(id)
                .map(|child| child.attrs.layer.unwrap_or(0))
                .unwrap_or(0)
        });
        children
    }

    /// Whether `id` is `ancestor` or lives inside its subtree.
    pub fn is_within(&self, id: NodeId, ancestor: NodeId) -> bool {
        let mut current = Some(id);
        while let Some(node) = current {
            if node == ancestor {
                return true;
            }
            current = self.nodes.get(&node).and_then(|node| node.parent);
        }
        false
    }

    /// The topmost visible focus trap, if any.
    pub fn topmost_focus_trap(&self) -> Option<NodeId> {
        self.paint_order().into_iter().rev().find(|id| {
            self.nodes
                .get(id)
                .map(|node| node.attrs.focus_trap)
                .unwrap_or(false)
        })
    }

    pub fn find_first_tag(&self, id: NodeId, tag: &str) -> Option<NodeId> {
        let node = self.nodes.get(&id)?;
        if node.tag == tag {
            return Some(id);
        }
        for child in &node.children {
            if let Some(found) = self.find_first_tag(*child, tag) {
                return Some(found);
            }
        }
        None
    }
}
