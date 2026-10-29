#![cfg(test)]

use super::*;

#[test]
fn builds_and_removes_trees() {
    let mut tree = NodeTree::new();
    let a = tree.insert_node("box", None);
    let b = tree.insert_node("text", None);
    let t = tree.insert_node("#text", Some("hello".into()));
    tree.append_child(tree.root_id(), a);
    tree.append_child(a, b);
    tree.append_child(b, t);

    assert_eq!(tree.collect_text(a), "hello");
    assert_eq!(tree.children_of(a), &[b]);

    tree.remove(b);
    assert!(tree.get(b).is_none());
    assert!(tree.get(t).is_none());
    assert!(tree.children_of(a).is_empty());
}
