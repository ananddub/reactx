use crate::harness::Harness;
use reactx::prelude::*;

fn counter_app() -> Element {
    let mut clicks = use_signal(|| 0u32);
    let mut doubles = use_signal(|| 0u32);
    rsx! {
        div { style: "direction:column",
            button {
                onclick: move |_| clicks += 1,
                ondoubleclick: move |_| doubles += 1,
                "hit"
            }
            span { "clicks {clicks()} doubles {doubles()}" }
        }
    }
}

fn status(harness: &Harness) -> String {
    let id = harness.node_where(|node| {
        node.tag == "#text"
            && node
                .text
                .as_deref()
                .is_some_and(|t| t.starts_with("clicks"))
    });
    harness.node_text(id)
}

#[test]
fn rapid_clicks_keep_firing() {
    let mut harness = Harness::new(counter_app);
    let button = harness.node_where(|node| node.tag == "button");
    let rect = harness.renderer.layout[&button];
    let (x, y) = (rect.x, rect.y);

    for expected in 1..=30u32 {
        harness.mouse_down(x, y, MouseButtonKind::Left);
        harness.mouse_up(x, y, MouseButtonKind::Left);
        assert!(
            status(&harness).starts_with(&format!("clicks {expected} ")),
            "click {expected} lost: {}",
            status(&harness)
        );
    }
    assert_eq!(status(&harness), "clicks 30 doubles 15");
}
