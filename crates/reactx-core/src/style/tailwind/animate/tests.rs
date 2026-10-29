//! Unit tests for the animation utilities.

use crate::style::{ClassState, StyleProps};

fn styled(classes: &str) -> StyleProps {
    styled_at(classes, ClassState::default())
}

fn styled_at(classes: &str, state: ClassState) -> StyleProps {
    let mut props = StyleProps::default();
    props.apply_classes(classes, &state);
    props
}

fn custom<'a>(props: &'a StyleProps, key: &str) -> Option<&'a str> {
    props.custom.get(key).map(String::as_str)
}

#[test]
fn effect_duration_easing_and_delay() {
    let props = styled("animate-fade duration-300 delay-150 ease-quad-out");
    assert_eq!(custom(&props, "animate"), Some("fade"));
    assert_eq!(custom(&props, "animate_duration"), Some("300"));
    assert_eq!(custom(&props, "animate_delay"), Some("150"));
    assert_eq!(custom(&props, "animate_easing"), Some("quad-out"));
}

#[test]
fn exit_repeat_ping_pong_and_stagger() {
    let props = styled(
        "animate-exit-slide-out animate-repeat-forever animate-ping-pong \
         animate-index-2 animate-stagger-80",
    );
    assert_eq!(custom(&props, "animate_exit"), Some("slide-out"));
    assert_eq!(custom(&props, "animate_repeat"), Some("forever"));
    assert_eq!(custom(&props, "animate_ping_pong"), Some("1"));
    assert_eq!(custom(&props, "animate_index"), Some("2"));
    assert_eq!(custom(&props, "animate_stagger"), Some("80"));
}

#[test]
fn colors_direction_and_bracketed_values() {
    let props = styled(
        "animate-slide-in animate-direction-up animate-color-red-500 \
         animate-gradient-[12]",
    );
    assert_eq!(custom(&props, "animate"), Some("slide-in"));
    assert_eq!(custom(&props, "animate_direction"), Some("up"));
    assert_eq!(custom(&props, "animate_color"), Some("#ef4444"));
    assert_eq!(custom(&props, "animate_gradient"), Some("12"));

    let props = styled("animate-[fade-in,dissolve] duration-[450ms]");
    assert_eq!(custom(&props, "animate"), Some("fade-in,dissolve"));
    assert_eq!(custom(&props, "animate_duration"), Some("450ms"));
}

#[test]
fn tailwind_easing_aliases() {
    assert_eq!(
        custom(&styled("ease-in"), "animate_easing"),
        Some("cubic-in")
    );
    assert_eq!(
        custom(&styled("ease-out"), "animate_easing"),
        Some("cubic-out")
    );
    assert_eq!(
        custom(&styled("ease-in-out"), "animate_easing"),
        Some("cubic-in-out")
    );
    assert_eq!(
        custom(&styled("ease-linear"), "animate_easing"),
        Some("linear")
    );
}

#[test]
fn tailwind_presets_loop_forever() {
    let pulse = styled("animate-pulse");
    assert_eq!(custom(&pulse, "animate"), Some("fade"));
    assert_eq!(custom(&pulse, "animate_repeat"), Some("forever"));
    assert_eq!(custom(&pulse, "animate_ping_pong"), Some("1"));

    let bounce = styled("animate-bounce");
    assert_eq!(custom(&bounce, "animate"), Some("slide-in-bottom"));
    assert_eq!(custom(&bounce, "animate_ping_pong"), Some("1"));

    let spin = styled("animate-spin");
    assert_eq!(custom(&spin, "animate"), Some("hsl-shift"));
    assert_eq!(custom(&spin, "animate_repeat"), Some("forever"));
    assert_eq!(custom(&spin, "animate_ping_pong"), None);

    let ping = styled("animate-ping");
    assert_eq!(custom(&ping, "animate"), Some("explode"));
    assert_eq!(custom(&ping, "animate_repeat"), Some("forever"));
}

#[test]
fn animate_none_clears_effects_and_variants_apply() {
    let props = styled("animate-fade animate-exit-fade-out animate-none");
    assert_eq!(custom(&props, "animate"), None);
    assert_eq!(custom(&props, "animate_exit"), None);

    let props = styled("animate-pulse animate-none");
    assert_eq!(custom(&props, "animate_repeat"), None);
    assert_eq!(custom(&props, "animate_ping_pong"), None);

    let narrow = styled("md:animate-dissolve");
    assert_eq!(custom(&narrow, "animate"), None);
    let wide = styled_at(
        "md:animate-dissolve",
        ClassState {
            width: 80,
            ..ClassState::default()
        },
    );
    assert_eq!(custom(&wide, "animate"), Some("dissolve"));
}
