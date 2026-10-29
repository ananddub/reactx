//! Structural mutations on [`NodeTree`].

use super::*;

impl NodeTree {
    /// Insert a detached node and return its id.
    pub fn insert_node(&mut self, tag: impl Into<String>, text: Option<String>) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        self.nodes.insert(
            id,
            TuiNode {
                id,
                tag: tag.into(),
                text,
                ..Default::default()
            },
        );
        id
    }

    pub fn bind_element(&mut self, dioxus_id: ElementId, node_id: NodeId) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            if let Some(previous) = node.dioxus_id {
                self.element_map.remove(&previous);
            }
            node.dioxus_id = Some(dioxus_id);
        }
        self.element_map.insert(dioxus_id, node_id);
    }

    pub fn node_for_element(&self, dioxus_id: ElementId) -> Option<NodeId> {
        self.element_map.get(&dioxus_id).copied()
    }

    pub fn children_of(&self, id: NodeId) -> &[NodeId] {
        self.nodes
            .get(&id)
            .map(|node| node.children.as_slice())
            .unwrap_or(&[])
    }

    pub fn element_for_node(&self, node_id: NodeId) -> Option<ElementId> {
        self.nodes.get(&node_id).and_then(|node| node.dioxus_id)
    }

    pub fn detach(&mut self, child: NodeId) {
        if child == self.root {
            return;
        }
        let parent = self.nodes.get(&child).and_then(|node| node.parent);
        if let Some(parent) = parent {
            if let Some(parent_node) = self.nodes.get_mut(&parent) {
                parent_node.children.retain(|c| *c != child);
            }
        }
        if let Some(node) = self.nodes.get_mut(&child) {
            node.parent = None;
        }
    }

    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        if parent == child || !self.nodes.contains_key(&parent) || !self.nodes.contains_key(&child)
        {
            return;
        }
        self.detach(child);
        if let Some(node) = self.nodes.get_mut(&parent) {
            node.children.push(child);
        }
        if let Some(node) = self.nodes.get_mut(&child) {
            node.parent = Some(parent);
        }
    }

    pub fn insert_child_at(&mut self, parent: NodeId, index: usize, child: NodeId) {
        if parent == child || !self.nodes.contains_key(&parent) || !self.nodes.contains_key(&child)
        {
            return;
        }
        self.detach(child);
        let index = index.min(
            self.nodes
                .get(&parent)
                .map(|node| node.children.len())
                .unwrap_or(0),
        );
        if let Some(node) = self.nodes.get_mut(&parent) {
            node.children.insert(index, child);
        }
        if let Some(node) = self.nodes.get_mut(&child) {
            node.parent = Some(parent);
        }
    }

    /// Insert `child` directly before `reference`.
    pub fn insert_child_before(&mut self, reference: NodeId, child: NodeId) {
        let parent = self.nodes.get(&reference).and_then(|node| node.parent);
        let Some(parent) = parent else {
            return;
        };
        let index = self
            .nodes
            .get(&parent)
            .and_then(|node| node.children.iter().position(|c| *c == reference))
            .unwrap_or(usize::MAX);
        if index == usize::MAX {
            self.append_child(parent, child);
        } else {
            self.insert_child_at(parent, index, child);
        }
    }

    /// Replace `target` in its parent's child list with `replacements`.
    pub fn replace_in_parent(&mut self, target: NodeId, replacements: &[NodeId]) {
        let Some(parent) = self.nodes.get(&target).and_then(|node| node.parent) else {
            return;
        };
        let Some(index) = self
            .nodes
            .get(&parent)
            .and_then(|node| node.children.iter().position(|c| *c == target))
        else {
            return;
        };
        for replacement in replacements {
            self.detach(*replacement);
        }
        if let Some(parent_node) = self.nodes.get_mut(&parent) {
            parent_node
                .children
                .splice(index..=index, replacements.iter().copied());
        }
        for replacement in replacements {
            if let Some(node) = self.nodes.get_mut(replacement) {
                node.parent = Some(parent);
            }
        }
        self.detach(target);
    }

    /// Remove a node and its entire subtree.
    pub fn remove(&mut self, id: NodeId) {
        if id == self.root {
            return;
        }
        self.detach(id);
        self.remove_subtree(id);
    }

    fn remove_subtree(&mut self, id: NodeId) {
        let children = match self.nodes.get(&id) {
            Some(node) => node.children.clone(),
            None => return,
        };
        for child in children {
            self.remove_subtree(child);
        }
        if let Some(node) = self.nodes.remove(&id) {
            if let Some(dioxus_id) = node.dioxus_id {
                self.element_map.remove(&dioxus_id);
            }
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}
