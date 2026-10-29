use crate::harness::Harness;
use reactx::prelude::*;

fn form_app() -> Element {
    let mut value = use_signal(String::new);
    jsx! {
        <div style="direction:column">
            <Input value={value()} placeholder="type here" on_input=move |next| value.set(next) />
            <span>value: {value}</span>
        </div>
    }
}

#[test]
fn input_attributes_and_signal_interpolation() {
    let mut harness = Harness::new(form_app);
    let input = harness.node_where(|node| node.tag == "input");
    harness.renderer.focus.set_focus(Some(input));
    harness.type_text("jsx");
    assert!(
        harness.screen().contains("value: jsx"),
        "{}",
        harness.screen()
    );
}

#[test]
fn typed_spaces_reach_input() {
    let mut harness = Harness::new(form_app);
    let input = harness.node_where(|node| node.tag == "input");
    harness.renderer.focus.set_focus(Some(input));
    harness.type_text("a b");
    assert!(
        harness.screen().contains("value: a b"),
        "{}",
        harness.screen()
    );
}
