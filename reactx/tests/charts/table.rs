use crate::harness::{any_cell, Harness};
use ratatui::style::Color;
use reactx::prelude::*;

fn table_app() -> Element {
    jsx! {
        <div style="direction:column;width:60">
            <Table
                headers={vec!["PID".to_string(), "Process".to_string(), "CPU".to_string()]}
                rows={vec![
                    vec!["1".to_string(), "systemd".to_string(), "0.1%".to_string()],
                    vec!["104".to_string(), "reactx-app".to_string(), "2.4%".to_string()],
                ]}
                selected_row={Some(1usize)}
                title="Processes"
            />
        </div>
    }
}

#[test]
fn table_renders_headers_rows_and_highlight() {
    let harness = Harness::new(table_app);
    let screen = harness.screen();
    assert!(screen.contains("PID"), "{screen}");
    assert!(screen.contains("Process"), "{screen}");
    assert!(screen.contains("systemd"), "{screen}");
    assert!(screen.contains("reactx-app"), "{screen}");
    assert!(
        any_cell(&harness, |cell| cell.bg == Color::Rgb(69, 71, 90)),
        "selected row must be highlighted; screen:\n{screen}"
    );
}
