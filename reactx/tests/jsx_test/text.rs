use crate::harness::Harness;
use reactx::prelude::*;

fn greeting_app() -> Element {
    let name = "world";
    jsx! {
        <div style="direction:column">
            <span>Hello, {name}!</span>
            <span>50 % done</span>
            <!-- comments are ignored -->
        </div>
    }
}

#[test]
fn text_spacing_around_interpolation() {
    let harness = Harness::new(greeting_app);
    let screen = harness.screen();
    assert!(screen.contains("Hello, world!"), "{screen}");
    assert!(screen.contains("50% done"), "{screen}");
    assert!(
        !screen.contains("Hello ,"),
        "punctuation must stay attached"
    );
    assert!(!screen.contains("50 %"), "percent must stay attached");
}

fn sentences_app() -> Element {
    jsx! {
        <div style="direction:column">
            <span>First. Second. Third.</span>
        </div>
    }
}

#[test]
fn sentence_spacing_survives_cleanup() {
    let harness = Harness::new(sentences_app);
    assert!(
        harness.screen().contains("First. Second. Third."),
        "{}",
        harness.screen()
    );
}
