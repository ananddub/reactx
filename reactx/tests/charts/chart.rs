use crate::harness::{any_cell, Harness};
use reactx::prelude::*;

fn chart_app() -> Element {
    let dataset = ChartDataset {
        name: "Server A".into(),
        data: vec![(0.0, 15.0), (1.0, 22.0), (2.0, 18.0), (3.0, 35.0)],
        color: Some("#a6e3a1".into()),
        marker: "braille".into(),
    };
    jsx! {
        <div style="direction:column;width:60">
            <Chart
                datasets={vec![dataset]}
                x_bounds={(0.0, 3.0)}
                y_bounds={(0.0, 40.0)}
                title="Latency"
            />
        </div>
    }
}

#[test]
fn chart_draws_braille_points() {
    let harness = Harness::new(chart_app);
    let screen = harness.screen();
    assert!(
        any_cell(&harness, |cell| cell
            .symbol()
            .chars()
            .any(|ch| ('\u{2800}'..='\u{28FF}').contains(&ch))),
        "expected braille glyphs; screen:\n{screen}"
    );
}
