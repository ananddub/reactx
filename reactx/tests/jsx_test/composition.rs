use crate::harness::Harness;
use reactx::prelude::*;

fn fragment_app() -> Element {
    jsx! {
        <>
            <span>first</span>
            <span>second</span>
        </>
    }
}

#[test]
fn fragments_flatten_into_multiple_roots() {
    let harness = Harness::new(fragment_app);
    let screen = harness.screen();
    assert!(screen.contains("first"), "{screen}");
    assert!(screen.contains("second"), "{screen}");
}

fn nested_app() -> Element {
    let mut count = use_signal(|| 0);
    jsx! {
        <div style="direction:column">
            {jsx! { <span>nested {count}</span> }}
            <Button on_click=move |_| count += 1>Bump</Button>
        </div>
    }
}

#[test]
fn nested_jsx_expressions_rerender() {
    let mut harness = Harness::new(nested_app);
    assert!(
        harness.screen().contains("nested 0"),
        "{}",
        harness.screen()
    );
    harness.click_text("button", "Bump");
    assert!(
        harness.screen().contains("nested 1"),
        "{}",
        harness.screen()
    );
}

fn injected_app() -> Element {
    let badge = rsx! { span { "injected badge" } };
    jsx! {
        <div style="direction:column">
            {badge}
            <span>after injection</span>
        </div>
    }
}

#[test]
fn elements_interpolate_directly() {
    let harness = Harness::new(injected_app);
    let screen = harness.screen();
    assert!(screen.contains("injected badge"), "{screen}");
    assert!(screen.contains("after injection"), "{screen}");
}

fn list_app() -> Element {
    let items = ["alpha", "beta", "gamma"];
    jsx! {
        <div style="direction:column">
            {items.into_iter().map(|item| jsx! { <span key="{item}">{item}</span> })}
        </div>
    }
}

#[test]
fn iterator_children_render_in_order() {
    let harness = Harness::new(list_app);
    let screen = harness.screen();
    assert!(screen.contains("alpha"), "{screen}");
    assert!(screen.contains("beta"), "{screen}");
    assert!(screen.contains("gamma"), "{screen}");
}

fn toggle_app() -> Element {
    let mut on = use_signal(|| true);
    jsx! {
        <div style="direction:column">
            {if on() {
                jsx! { <span>lamp on</span> }
            } else {
                jsx! { <span>lamp off</span> }
            }}
            <Button on_click=move |_| on.set(!on())>Flip</Button>
        </div>
    }
}

#[test]
fn conditional_interpolation_switches() {
    let mut harness = Harness::new(toggle_app);
    assert!(harness.screen().contains("lamp on"), "{}", harness.screen());
    harness.click_text("button", "Flip");
    assert!(
        harness.screen().contains("lamp off"),
        "{}",
        harness.screen()
    );
}
