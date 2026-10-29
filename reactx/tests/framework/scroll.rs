use crate::harness::Harness;
use reactx::prelude::*;

static LAST_ROW: Store<usize> = Store::new("framework_last_row", || usize::MAX);

fn clickable_list_app() -> Element {
    let mut offset = use_signal(|| 0u16);
    let style = format!("direction:column;height:4;overflow:hidden;scroll-y:{offset}");
    rsx! {
        div { style: "direction:column",
            div {
                style: style,
                onwheel: move |event: Event<WheelData>| {
                    let delta = event.delta().strip_units().y as i32;
                    offset.set((offset() as i32 + delta).clamp(0, 6) as u16);
                },
                for index in 0..10usize {
                    div {
                        key: "{index}",
                        style: "height:1",
                        onclick: move |_| LAST_ROW.with_mut(|last| *last = index),
                        "row {index}"
                    }
                }
            }
        }
    }
}

#[test]
fn clicks_hit_rows_after_scrolling() {
    let mut harness = Harness::new(clickable_list_app);
    harness.click(2, 1);
    assert_eq!(harness.dom.in_runtime(|| LAST_ROW.with(|last| *last)), 1);

    harness.wheel(2, 1, 0, 3);
    assert!(harness.screen().contains("row 4"), "{}", harness.screen());
    assert!(
        !harness.screen().contains("row 0"),
        "the view must be clipped:\n{}",
        harness.screen()
    );

    harness.click(2, 1);
    assert_eq!(
        harness.dom.in_runtime(|| LAST_ROW.with(|last| *last)),
        4,
        "a click after scrolling must hit the row drawn under the pointer:\n{}",
        harness.screen()
    );
}

fn scroll_app() -> Element {
    let mut offset = use_signal(|| 0u16);
    let style = format!("direction:column;height:5;overflow:hidden;scroll-y:{offset}");
    rsx! {
        div { style: "direction:column",
            div {
                style: style,
                onwheel: move |event: Event<WheelData>| {
                    let delta = event.delta().strip_units().y as i32;
                    let next = (offset() as i32 + delta).clamp(0, 15);
                    offset.set(next as u16);
                },
                for index in 0..20 {
                    span { key: "{index}", "row {index}" }
                }
            }
        }
    }
}

#[test]
fn scroll_container_clips_and_follows_the_wheel() {
    let mut harness = Harness::new(scroll_app);
    let screen = harness.screen();
    assert!(screen.contains("row 0"), "{screen}");
    assert!(screen.contains("row 4"), "{screen}");
    assert!(
        !screen.contains("row 5"),
        "rows past the clip must be hidden:\n{screen}"
    );

    let container = harness.node_where(|node| node.attrs.overflow.is_some());
    let rect = harness.renderer.layout[&container];

    harness.wheel(rect.x + 1, rect.y + 1, 0, 2);
    let screen = harness.screen();
    assert!(
        !screen.contains("row 0"),
        "scrolled-out row is still painted:\n{screen}"
    );
    assert!(
        !screen.contains("row 1"),
        "scrolled-out row is still painted:\n{screen}"
    );
    assert!(screen.contains("row 2"), "{screen}");
    assert!(screen.contains("row 6"), "{screen}");
    assert!(!screen.contains("row 7"), "{screen}");

    assert_eq!(
        harness.renderer.tree.get(container).unwrap().attrs.scroll_y,
        2.0
    );
}
