use crate::harness::Harness;
use reactx::prelude::*;

static MOUSE_LOG: Store<String> = Store::new("framework_mouse_log", String::new);

fn mouse_app() -> Element {
    jsx! {
        <div style="direction:column">
            <div
                style="height:2"
                onmousedown={move |_| MOUSE_LOG.with_mut(|log| log.push_str("down;"))}
                onmouseup={move |_| MOUSE_LOG.with_mut(|log| log.push_str("up;"))}
                onmousemove={move |_| MOUSE_LOG.with_mut(|log| log.push_str("move;"))}
                onmousedrag={move |_| MOUSE_LOG.with_mut(|log| log.push_str("drag;"))}
                onmouseover={move |_| MOUSE_LOG.with_mut(|log| log.push_str("over;"))}
                onmouseout={move |_| MOUSE_LOG.with_mut(|log| log.push_str("out;"))}
            >alpha</div>
            <div
                style="height:2"
                onmouseover={move |_| MOUSE_LOG.with_mut(|log| log.push_str("over-beta;"))}
            >beta</div>
        </div>
    }
}

fn mouse_log(harness: &Harness) -> String {
    harness.dom.in_runtime(|| MOUSE_LOG.peek())
}

#[test]
fn mouse_buttons_hover_and_drag_raise_events() {
    let mut harness = Harness::new(mouse_app);
    harness.dom.in_runtime(|| MOUSE_LOG.set(String::new()));

    let alpha =
        harness.node_where(|node| node.tag == "#text" && node.text.as_deref() == Some("alpha"));
    let beta =
        harness.node_where(|node| node.tag == "#text" && node.text.as_deref() == Some("beta"));
    let alpha_rect = harness.renderer.layout[&alpha];
    let beta_rect = harness.renderer.layout[&beta];
    let (x, y) = (alpha_rect.x, alpha_rect.y);

    harness.mouse_move(x, y);
    assert_eq!(mouse_log(&harness), "over;move;");
    harness.mouse_move(x, y);
    assert_eq!(mouse_log(&harness), "over;move;move;");

    harness.mouse_move(beta_rect.x, beta_rect.y);
    assert_eq!(
        mouse_log(&harness),
        "over;move;move;out;over-beta;",
        "moving between nodes must raise mouseout then mouseover"
    );

    harness.mouse_down(x, y, MouseButtonKind::Left);
    harness.mouse_up(x, y, MouseButtonKind::Right);
    harness.mouse_drag(x, y, MouseButtonKind::Left);
    assert!(
        mouse_log(&harness).ends_with("down;up;drag;"),
        "{:?}",
        mouse_log(&harness)
    );
}
