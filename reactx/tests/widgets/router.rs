use crate::harness::Harness;
use reactx::prelude::*;

fn Home() -> Element {
    rsx! {
        div { style: "direction:column",
            span { "home page" }
            Link { to: "/todo/7", "open todo" }
        }
    }
}

fn TodoPage() -> Element {
    let id = use_param("id").unwrap_or_default();
    rsx! { span { "todo {id}" } }
}

fn app() -> Element {
    rsx! {
        Router {
            routes: routes! {
                "/" => Home,
                "/todo/:id" => TodoPage,
            },
        }
    }
}

#[test]
fn link_click_navigates_and_captures_params() {
    let mut harness = Harness::new(app);
    assert!(harness.screen().contains("home page"));

    harness.click_text("text", "open todo");
    assert!(harness.screen().contains("todo 7"), "{}", harness.screen());
}
