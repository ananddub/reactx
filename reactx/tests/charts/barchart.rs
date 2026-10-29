use crate::harness::Harness;
use reactx::prelude::*;

fn barchart_app() -> Element {
    jsx! {
        <div style="direction:column;width:50">
            <BarChart
                data={vec![
                    ("Rust".to_string(), 95u64),
                    ("Go".to_string(), 80),
                    ("Python".to_string(), 65),
                ]}
                bar_width={Some(8)}
                color="#89b4fa"
                title="Benchmarks"
            />
        </div>
    }
}

#[test]
fn barchart_draws_labels_and_bars() {
    let harness = Harness::new(barchart_app);
    let screen = harness.screen();
    assert!(screen.contains("Rust"), "{screen}");
    assert!(screen.contains("Python"), "{screen}");
    assert!(screen.contains('█'), "{screen}");
}
