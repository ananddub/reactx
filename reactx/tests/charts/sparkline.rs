use crate::harness::Harness;
use reactx::prelude::*;

fn sparkline_app() -> Element {
    jsx! {
        <div style="direction:column;width:40">
            <Sparkline data={vec![0u64, 3, 6, 9, 12]} max={Some(12)} color="#a6e3a1" />
        </div>
    }
}

#[test]
fn sparkline_draws_blocks() {
    let harness = Harness::new(sparkline_app);
    let screen = harness.screen();
    assert!(
        "▁▂▃▄▅▆▇█".chars().any(|symbol| screen.contains(symbol)),
        "expected sparkline blocks; screen:\n{screen}"
    );
}
