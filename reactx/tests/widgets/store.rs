use crate::harness::Harness;
use reactx::prelude::*;

#[test]
fn store_rerenders_every_subscriber() {
    static COUNT: Store<i32> = Store::new("widget_test_count", || 0);

    fn app() -> Element {
        let count = COUNT.with(|count| *count);
        rsx! {
            div { style: "direction:column",
                span { "count {count}" }
                CountBadge {}
                button {
                    onclick: move |_| COUNT.with_mut(|count| *count += 1),
                    "bump"
                }
            }
        }
    }

    fn CountBadge() -> Element {
        let count = COUNT.with(|count| *count);
        rsx! { span { "badge {count}" } }
    }

    let mut harness = Harness::new(app);
    assert!(harness.screen().contains("count 0"));
    assert!(harness.screen().contains("badge 0"));

    harness.click_text("button", "bump");
    assert!(harness.screen().contains("count 1"));
    assert!(harness.screen().contains("badge 1"));
}
