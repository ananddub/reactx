//! Ink-style layout metrics: `useBoxMetrics` and `measureElement`.

use dioxus_core::VirtualDom;
use dioxus_signals::{GlobalSignal, Readable, Signal};
use rustc_hash::FxHashMap;

use crate::TuiRenderer;

/// Measured geometry of an element with a user `id`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoxMetrics {
    /// Outer width in cells, including borders.
    pub width: u16,
    /// Outer height in cells, including borders.
    pub height: u16,
    /// Offset from the parent's left edge.
    pub left: u16,
    /// Offset from the parent's top edge.
    pub top: u16,
    /// Width inside the border.
    pub client_width: u16,
    /// Height inside the border.
    pub client_height: u16,
    /// Whether a layout has measured this element yet.
    pub has_measured: bool,
}

/// Metrics of every element with a user `id`, refreshed after each frame.
pub static BOX_METRICS: GlobalSignal<FxHashMap<String, BoxMetrics>> =
    Signal::global(FxHashMap::default);

/// Measure the element with user id `id`, subscribing to future layout changes.
pub fn use_box_metrics(id: &str) -> BoxMetrics {
    let metrics = BOX_METRICS();
    metrics.get(id).copied().unwrap_or_default()
}

/// Measure the element with user id `id`. Unlike [`use_box_metrics`] this does
/// not subscribe, so it is safe outside render.
pub fn measure_element(id: &str) -> Option<BoxMetrics> {
    BOX_METRICS.peek().get(id).copied()
}

/// Recompute the metrics map from the last layout pass.
pub(crate) fn publish(dom: &VirtualDom, renderer: &TuiRenderer) {
    let mut next: FxHashMap<String, BoxMetrics> = FxHashMap::default();
    for node_id in renderer.tree.nodes_in_order() {
        let Some(node) = renderer.tree.get(node_id) else {
            continue;
        };
        let Some(user_id) = node.attrs.custom.get("id") else {
            continue;
        };
        let Some(rect) = renderer.layout.get(&node_id) else {
            continue;
        };
        let parent = node
            .parent
            .and_then(|parent| renderer.layout.get(&parent))
            .copied();
        let (left, top) = match parent {
            Some(parent) => (
                rect.x.saturating_sub(parent.x),
                rect.y.saturating_sub(parent.y),
            ),
            None => (rect.x, rect.y),
        };
        let bordered = crate::layout::effective_border(node).is_visible();
        let (client_width, client_height) = if bordered {
            (rect.width.saturating_sub(2), rect.height.saturating_sub(2))
        } else {
            (rect.width, rect.height)
        };
        next.insert(
            user_id.clone(),
            BoxMetrics {
                width: rect.width,
                height: rect.height,
                left,
                top,
                client_width,
                client_height,
                has_measured: true,
            },
        );
    }
    let unchanged = dom.in_runtime(|| *BOX_METRICS.peek() == next);
    if !unchanged {
        dom.in_runtime(|| BOX_METRICS.with_mut(|slot| *slot = next));
    }
}

#[cfg(test)]
mod tests {
    use dioxus::prelude::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    use super::*;

    fn app() -> Element {
        rsx! {
            div { style: "padding:1",
                div { id: "panel", style: "width:6;height:2", "hi" }
            }
        }
    }

    #[test]
    fn elements_are_measured_by_user_id() {
        let mut terminal = Terminal::new(TestBackend::new(20, 8)).unwrap();
        let mut dom = VirtualDom::new(app);
        let mut renderer = crate::TuiRenderer::new();
        dom.rebuild(&mut renderer);
        crate::terminal::set_terminal_size(&dom, 20, 8);
        dom.render_immediate(&mut renderer);
        renderer.paint(&mut terminal).unwrap();

        publish(&dom, &renderer);
        let metrics = dom
            .in_runtime(|| measure_element("panel"))
            .expect("panel measured");
        assert!(metrics.has_measured);
        assert_eq!((metrics.width, metrics.height), (6, 2));
        assert_eq!((metrics.left, metrics.top), (1, 1));
        assert_eq!((metrics.client_width, metrics.client_height), (6, 2));
    }
}
