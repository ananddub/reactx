//! Renderer internals shared by the Dioxus mutation handlers.

use dioxus_core::TemplateNode;
use reactx_core::NodeId;

use super::attrs::normalize_tag;
use super::TuiRenderer;

impl TuiRenderer {
    pub(super) fn resolve_element(&self, id: dioxus_core::ElementId) -> Option<NodeId> {
        if id.0 == 0 {
            Some(self.tree.root_id())
        } else {
            self.tree.node_for_element(id)
        }
    }

    pub(super) fn resolve_path(&self, start: NodeId, path: &[u8]) -> Option<NodeId> {
        let mut current = start;
        for index in path {
            current = *self.tree.children_of(current).get(*index as usize)?;
        }
        Some(current)
    }

    pub(super) fn pop_stack(&mut self, count: usize) -> Vec<NodeId> {
        let count = count.min(self.stack.len());
        self.stack.split_off(self.stack.len() - count)
    }

    pub(super) fn apply_text_attr(&mut self, node_id: NodeId, name: &str, value: Option<&str>) {
        let Some(node) = self.tree.get_mut(node_id) else {
            return;
        };
        let normalized = name.trim().to_ascii_lowercase().replace('-', "_");
        if normalized == "style" {
            let mut keys: Vec<String> = Vec::new();
            if let Some(value) = value {
                for declaration in value.split(';') {
                    if let Some((key, value)) = declaration.split_once(':') {
                        let key = key.trim();
                        if key.is_empty() {
                            continue;
                        }
                        node.attrs.apply_attr(key, value.trim());
                        keys.push(key.to_ascii_lowercase().replace('-', "_"));
                    }
                }
            }
            for stale in std::mem::take(&mut node.style_keys) {
                if !keys.contains(&stale) {
                    node.attrs.clear_attr(&stale);
                }
            }
            node.style_keys = keys;
            return;
        }
        match value {
            Some(value) => node.attrs.apply_attr(name, value),
            None => node.attrs.clear_attr(name),
        }
    }

    pub(super) fn instantiate_node(&mut self, template_node: &TemplateNode) -> NodeId {
        match template_node {
            TemplateNode::Element {
                tag,
                attrs,
                children,
                ..
            } => {
                let tag = normalize_tag(tag);
                let node_id = self.tree.insert_node(tag, None);
                for attr in attrs.iter() {
                    if let dioxus_core::TemplateAttribute::Static { name, value, .. } = attr {
                        self.apply_text_attr(node_id, name, Some(value));
                    }
                }
                let child_ids: Vec<NodeId> = children
                    .iter()
                    .map(|child| self.instantiate_node(child))
                    .collect();
                for child in child_ids {
                    self.tree.append_child(node_id, child);
                }
                node_id
            }
            TemplateNode::Text { text } => {
                self.tree.insert_node("#text", Some((*text).to_string()))
            }
            TemplateNode::Dynamic { .. } => self.tree.insert_node("#placeholder", None),
        }
    }
}
