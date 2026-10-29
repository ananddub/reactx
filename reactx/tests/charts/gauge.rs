use crate::harness::{any_cell, Harness};
use reactx::prelude::*;

fn gauge_app() -> Element {
    jsx! {
        <div style="direction:column;width:50">
            <Gauge value={0.82} label="82% Synced" border="rounded" title="Progress" />
            <Gauge value={45.0} use_line=true label="45%" />
        </div>
    }
}

#[test]
fn gauges_render_labels_and_fill() {
    let harness = Harness::new(gauge_app);
    let screen = harness.screen();
    assert!(screen.contains("82% Synced"), "{screen}");
    assert!(screen.contains("45%"), "{screen}");
    assert!(any_cell(&harness, |cell| cell.symbol() == "█"), "{screen}");
}
