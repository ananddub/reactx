use crate::harness::Harness;
use reactx::prelude::*;

fn size_app() -> Element {
    let (width, height) = use_terminal_size();
    rsx! {
        div { "size {width}x{height}" }
    }
}

#[test]
fn terminal_size_updates_on_resize() {
    let mut harness = Harness::new(size_app);
    harness.event(TuiEvent::Resize {
        width: 50,
        height: 20,
    });
    assert!(
        harness.screen().contains("size 50x20"),
        "{}",
        harness.screen()
    );

    harness.event(TuiEvent::Resize {
        width: 40,
        height: 10,
    });
    assert!(
        harness.screen().contains("size 40x10"),
        "{}",
        harness.screen()
    );
}
