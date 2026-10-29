use crate::harness::Harness;
use reactx::prelude::*;

#[test]
fn tabs_switch_selection() {
    fn app() -> Element {
        let mut selected = use_signal(|| 0usize);
        rsx! {
            div { style: "direction:column",
                Tabs {
                    tabs: vec!["One".to_string(), "Two".to_string()],
                    selected: selected(),
                    on_select: move |index| selected.set(index),
                }
                span { "selected {selected()}" }
            }
        }
    }

    let mut harness = Harness::new(app);
    assert!(harness.screen().contains("selected 0"));
    harness.click_text("button", "Two");
    assert!(harness.screen().contains("selected 1"));
}
