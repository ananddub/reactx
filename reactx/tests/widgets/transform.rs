use crate::harness::Harness;
use reactx::prelude::*;

fn app() -> Element {
    rsx! {
        ThemeProvider {
            Text {
                Transform {
                    transform: Callback::new(|text: String| text.to_uppercase()),
                    "hello world"
                }
            }
        }
    }
}

#[test]
fn transform_rewrites_child_text() {
    let harness = Harness::new(app);
    assert!(
        harness.screen().contains("HELLO WORLD"),
        "{}",
        harness.screen()
    );
}

fn nested_app() -> Element {
    rsx! {
        ThemeProvider {
            Text {
                Transform {
                    transform: Callback::new(|text: String| format!("[{text}]")),
                    Transform {
                        transform: Callback::new(|text: String| text.to_uppercase()),
                        "hi"
                    }
                }
            }
        }
    }
}

#[test]
fn transforms_compose() {
    let harness = Harness::new(nested_app);
    assert!(harness.screen().contains("[HI]"), "{}", harness.screen());
}
