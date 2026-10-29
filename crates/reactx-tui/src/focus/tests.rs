use super::*;

fn tree_with_focusables() -> (NodeTree, [NodeId; 3]) {
    let mut tree = NodeTree::new();
    let root = tree.root_id();
    let a = tree.insert_node("button", None);
    let b = tree.insert_node("button", None);
    let c = tree.insert_node("input", None);
    tree.append_child(root, a);
    tree.append_child(root, b);
    tree.append_child(root, c);
    (tree, [a, b, c])
}

#[test]
fn focus_ring_autofocuses_first_and_wraps() {
    let (tree, [a, b, c]) = tree_with_focusables();
    let mut focus = FocusManager::new();
    focus.rebuild(&tree);
    assert_eq!(focus.focused_id(), Some(a));

    focus.focus_next();
    assert_eq!(focus.focused_id(), Some(b));
    focus.focus_next();
    assert_eq!(focus.focused_id(), Some(c));
    focus.focus_next();
    assert_eq!(focus.focused_id(), Some(a));

    focus.focus_prev();
    assert_eq!(focus.focused_id(), Some(c));
}

#[test]
fn losing_the_focused_node_moves_focus() {
    let (mut tree, [a, b, _]) = tree_with_focusables();
    let mut focus = FocusManager::new();
    focus.rebuild(&tree);
    focus.set_focus(Some(b));

    tree.remove(b);
    focus.rebuild(&tree);
    assert_eq!(focus.focused_id(), Some(a));
}

#[test]
fn trap_restricts_ring_and_restores_focus() {
    let mut tree = NodeTree::new();
    let background = tree.insert_node("button", None);
    tree.append_child(tree.root_id(), background);
    let overlay = tree.insert_node("box", None);
    tree.append_child(tree.root_id(), overlay);
    let first = tree.insert_node("button", None);
    let second = tree.insert_node("button", None);
    tree.append_child(overlay, first);
    tree.append_child(overlay, second);

    let mut focus = FocusManager::new();
    focus.rebuild(&tree);
    assert_eq!(focus.focused_id(), Some(background));

    tree.get_mut(overlay).unwrap().attrs.focus_trap = true;
    focus.rebuild(&tree);
    assert_eq!(focus.trap(), Some(overlay));
    assert_eq!(focus.focused_id(), Some(first));
    assert_eq!(focus.order(), &[first, second]);

    focus.focus_next();
    assert_eq!(focus.focused_id(), Some(second));
    focus.focus_next();
    assert_eq!(focus.focused_id(), Some(first));

    tree.remove(overlay);
    focus.rebuild(&tree);
    assert_eq!(focus.trap(), None);
    assert_eq!(focus.focused_id(), Some(background));
}

#[test]
fn autofocus_wins_inside_a_trap() {
    let mut tree = NodeTree::new();
    let overlay = tree.insert_node("box", None);
    tree.get_mut(overlay).unwrap().attrs.focus_trap = true;
    tree.append_child(tree.root_id(), overlay);
    let first = tree.insert_node("button", None);
    let preferred = tree.insert_node("button", None);
    tree.get_mut(preferred).unwrap().attrs.autofocus = true;
    tree.append_child(overlay, first);
    tree.append_child(overlay, preferred);

    let mut focus = FocusManager::new();
    focus.rebuild(&tree);
    assert_eq!(focus.focused_id(), Some(preferred));
}
