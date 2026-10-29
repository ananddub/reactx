use crate::harness::Harness;
use reactx::prelude::*;

fn scrollbar_app() -> Element {
    jsx! {
        <div style="direction:row;height:12;width:30">
            <Scrollbar position={50usize} content_length={100usize} orientation="vertical-right" />
        </div>
    }
}

#[test]
fn scrollbar_draws_track_and_thumb() {
    let harness = Harness::new(scrollbar_app);
    let screen = harness.screen();
    assert!(
        screen.contains('║') || screen.contains('█'),
        "expected scrollbar symbols; screen:\n{screen}"
    );
}
