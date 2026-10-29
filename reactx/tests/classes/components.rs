use crate::harness::Harness;
use reactx::prelude::*;
use reactx::TextAlign;

fn widgets_app() -> Element {
    rsx! {
        ThemeProvider {
            Container { class: "p-4", "Panel" }
            Button { class: "text-center", on_click: move |_| {}, "Press" }
            Text { class: "bold", "Label" }
        }
    }
}

#[test]
fn widgets_forward_classes_to_their_elements() {
    let harness = Harness::new(widgets_app);

    let container = harness.node_where(|node| node.class.as_deref() == Some("p-4"));
    assert_eq!(harness.attrs(container).padding.top, 4.0);

    let button = harness.node_where(|node| node.tag == "button");
    assert_eq!(harness.attrs(button).text_align, Some(TextAlign::Center));

    let text =
        harness.node_where(|node| node.tag == "text" && harness.node_text(node.id) == "Label");
    assert!(harness.attrs(text).bold);
}

fn precedence_app() -> Element {
    rsx! {
        ThemeProvider {
            Container { padding: "1", class: "p-4", "Inline" }
            Button { class: "bg-red-500", on_click: move |_| {}, "Tinted" }
        }
    }
}

#[test]
fn inline_style_beats_classes() {
    let harness = Harness::new(precedence_app);

    let container = harness.node_where(|node| node.class.as_deref() == Some("p-4"));
    assert_eq!(harness.attrs(container).padding.top, 1.0);

    let button = harness.node_where(|node| node.tag == "button");
    assert_ne!(
        harness.attrs(button).bg,
        Some(TuiColor::Rgb(0xef, 0x44, 0x44))
    );
}

fn data_widgets_app() -> Element {
    rsx! {
        ThemeProvider {
            Gauge { value: 0.5, class: "w-20", border: "rounded" }
            Scrollbar { position: 1, content_length: 10, class: "h-5" }
        }
    }
}

#[test]
fn data_widgets_forward_classes_to_their_elements() {
    let harness = Harness::new(data_widgets_app);
    let gauge = harness.node_where(|node| node.class.as_deref() == Some("w-20"));
    assert_eq!(harness.attrs(gauge).width, Some(Dim::Cells(20.0)));

    let bar = harness.node_where(|node| node.class.as_deref() == Some("h-5"));
    assert_eq!(harness.attrs(bar).height, Some(Dim::Cells(5.0)));
}

fn list_app() -> Element {
    rsx! {
        ThemeProvider {
            List { class: "gap-1",
                ListItem { class: "underline", "row" }
            }
        }
    }
}

#[test]
fn list_and_items_accept_classes() {
    let harness = Harness::new(list_app);
    let list = harness.node_where(|node| node.class.as_deref() == Some("gap-1"));
    assert_eq!(harness.attrs(list).gap, Some((1.0, 1.0)));

    let item = harness.node_where(|node| node.class.as_deref() == Some("underline"));
    assert!(harness.attrs(item).underline);
}
