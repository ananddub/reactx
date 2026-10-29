//! The Dioxus `WriteMutations` implementation.

use dioxus_core::{AttributeValue, ElementId, Template, WriteMutations};

use super::attrs::attribute_to_string;
use super::TuiRenderer;

impl WriteMutations for TuiRenderer {
    fn append_children(&mut self, id: ElementId, m: usize) {
        let Some(parent) = self.resolve_element(id) else {
            return;
        };
        let children = self.pop_stack(m);
        for child in children {
            self.tree.append_child(parent, child);
        }
    }

    fn assign_node_id(&mut self, path: &'static [u8], id: ElementId) {
        let Some(top) = self.stack.last().copied() else {
            return;
        };
        if let Some(node) = self.resolve_path(top, path) {
            self.tree.bind_element(id, node);
        }
    }

    fn create_placeholder(&mut self, id: ElementId) {
        let node = self.tree.insert_node("#placeholder", None);
        self.tree.bind_element(id, node);
        self.stack.push(node);
    }

    fn create_text_node(&mut self, value: &str, id: ElementId) {
        let node = self.tree.insert_node("#text", Some(value.to_string()));
        self.tree.bind_element(id, node);
        self.stack.push(node);
    }

    fn load_template(&mut self, template: Template, index: usize, id: ElementId) {
        let Some(root) = template.roots.get(index) else {
            return;
        };
        let node = self.instantiate_node(root);
        self.tree.bind_element(id, node);
        self.stack.push(node);
    }

    fn replace_node_with(&mut self, id: ElementId, m: usize) {
        let Some(target) = self.resolve_element(id) else {
            return;
        };
        let replacements = self.pop_stack(m);
        self.tree.replace_in_parent(target, &replacements);
        self.tree.remove(target);
    }

    fn replace_placeholder_with_nodes(&mut self, path: &'static [u8], m: usize) {
        // The created nodes sit on top of the stack; the path is relative to the
        // template root that is revealed once they are popped.
        let replacements = self.pop_stack(m);
        let Some(top) = self.stack.last().copied() else {
            return;
        };
        let Some(target) = self.resolve_path(top, path) else {
            return;
        };
        self.tree.replace_in_parent(target, &replacements);
        self.tree.remove(target);
    }

    fn insert_nodes_after(&mut self, id: ElementId, m: usize) {
        let Some(target) = self.resolve_element(id) else {
            return;
        };
        let children = self.pop_stack(m);
        let Some(parent) = self.tree.get(target).and_then(|node| node.parent) else {
            return;
        };
        let index = self
            .tree
            .children_of(parent)
            .iter()
            .position(|child| *child == target)
            .map(|index| index + 1)
            .unwrap_or_else(|| self.tree.children_of(parent).len());
        for (offset, child) in children.into_iter().enumerate() {
            self.tree.insert_child_at(parent, index + offset, child);
        }
    }

    fn insert_nodes_before(&mut self, id: ElementId, m: usize) {
        let Some(target) = self.resolve_element(id) else {
            return;
        };
        for child in self.pop_stack(m) {
            self.tree.insert_child_before(target, child);
        }
    }

    fn set_attribute(
        &mut self,
        name: &'static str,
        _ns: Option<&'static str>,
        value: &AttributeValue,
        id: ElementId,
    ) {
        let Some(node_id) = self.resolve_element(id) else {
            return;
        };
        let value = attribute_to_string(value);
        self.apply_text_attr(node_id, name, value.as_deref());
    }

    fn set_node_text(&mut self, value: &str, id: ElementId) {
        if let Some(node_id) = self.resolve_element(id) {
            if let Some(node) = self.tree.get_mut(node_id) {
                node.text = Some(value.to_string());
            }
        }
    }

    fn create_event_listener(&mut self, name: &'static str, id: ElementId) {
        if let Some(node_id) = self.resolve_element(id) {
            if let Some(node) = self.tree.get_mut(node_id) {
                node.events.insert(name.to_string());
            }
        }
    }

    fn remove_event_listener(&mut self, name: &'static str, id: ElementId) {
        if let Some(node_id) = self.resolve_element(id) {
            if let Some(node) = self.tree.get_mut(node_id) {
                node.events.remove(name);
            }
        }
    }

    fn remove_node(&mut self, id: ElementId) {
        if let Some(node_id) = self.resolve_element(id) {
            self.tree.remove(node_id);
        }
    }

    fn push_root(&mut self, id: ElementId) {
        if let Some(node_id) = self.resolve_element(id) {
            self.stack.push(node_id);
        }
    }
}
