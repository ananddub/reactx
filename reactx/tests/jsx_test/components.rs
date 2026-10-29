use crate::harness::Harness;
use reactx::prelude::*;

fn counter_app() -> Element {
    let mut count = use_signal(|| 0);
    jsx! {
        <Container border="rounded" border-color="#89b4fa" padding="1">
            <Text color="#a6e3a1" bold>JSX count: {count}</Text>
            <Button on_click=move |_| count += 1>Increment</Button>
        </Container>
    }
}

#[test]
fn nested_elements_components_events_and_hyphenated_attributes() {
    let mut harness = Harness::new(counter_app);
    assert!(
        harness.screen().contains("JSX count: 0"),
        "{}",
        harness.screen()
    );

    let text_node = harness.node_where(|node| {
        node.tag == "text"
            && harness
                .renderer
                .tree
                .collect_text(node.id)
                .contains("JSX count")
    });
    assert!(harness.renderer.tree.get(text_node).unwrap().attrs.bold);

    harness.click_text("button", "Increment");
    assert!(
        harness.screen().contains("JSX count: 1"),
        "{}",
        harness.screen()
    );
    harness.click_text("button", "Increment");
    assert!(
        harness.screen().contains("JSX count: 2"),
        "{}",
        harness.screen()
    );
}
