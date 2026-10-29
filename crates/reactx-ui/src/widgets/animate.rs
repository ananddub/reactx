//! Wraps children in a box that plays a one-shot animation.

use dioxus::prelude::*;

use crate::widgets::style_push;

/// Wraps any subtree and animates it on mount, and again whenever `trigger`
/// changes.
///
/// ```ignore
/// jsx! {
///     <Animate effect="dissolve" duration={400} trigger={*shown}>
///         <Table headers={headers} rows={rows} />
///     </Animate>
/// }
/// ```
///
/// The wrapper is a plain flex box, so give it a `class` when the extra box
/// would change the surrounding layout (for example `class="w-full"`).
#[component]
pub fn Animate(
    /// `fade`, `fade-out`, `dissolve`, `coalesce`, `slide-in`, `slide-out`,
    /// `sweep-in`, `sweep-out`, `explode`, `hsl-shift` or `glitch`.
    #[props(default = "fade".to_string())]
    effect: String,
    /// Animation length in milliseconds.
    #[props(default = 480)]
    duration: u32,
    /// Easing curve, e.g. `linear`, `quad-out`, `cubic-in-out`, `sine-in-out`.
    #[props(default = "quad-out".to_string())]
    easing: String,
    /// Direction for slide/sweep effects: `left`, `right`, `up`, `down`.
    #[props(default)]
    direction: String,
    /// Fill colour (names, `#hex`, `rgb()`); defaults to black.
    #[props(default)]
    color: String,
    /// Increment this value to replay the animation.
    #[props(default)]
    trigger: u64,
    /// Effect played when the wrapped subtree is removed (for example when a
    /// row is deleted). A comma-separated sequence works too.
    #[props(default)]
    exit: String,
    /// Repeat the animation: `2`, `3x` or `forever`.
    #[props(default)]
    repeat: String,
    /// Play the effect forwards and then backwards.
    #[props(default)]
    ping_pong: bool,
    /// Delay before the animation starts, in milliseconds.
    #[props(default)]
    delay: u32,
    /// Position in a staggered group; waits `index * step` milliseconds.
    #[props(default)]
    index: u32,
    /// Milliseconds between staggered siblings.
    #[props(default = 100)]
    step: u32,
    /// Tailwind-style utility classes for the wrapper box.
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    let mut style = String::new();
    style_push(&mut style, "animate", &effect);
    style_push(&mut style, "animate-duration", &duration.to_string());
    style_push(&mut style, "animate-easing", &easing);
    style_push(&mut style, "animate-direction", &direction);
    style_push(&mut style, "animate-color", &color);
    style_push(&mut style, "animate-key", &trigger.to_string());
    if !exit.is_empty() {
        style_push(&mut style, "animate-exit", &exit);
    }
    if !repeat.is_empty() {
        style_push(&mut style, "animate-repeat", &repeat);
    }
    if ping_pong {
        style_push(&mut style, "animate-ping-pong", "1");
    }
    if delay > 0 {
        style_push(&mut style, "animate-delay", &delay.to_string());
    }
    if index > 0 {
        style_push(&mut style, "animate-index", &index.to_string());
        style_push(&mut style, "animate-stagger", &step.to_string());
    }

    rsx! {
        div { class: class, style: style, {children} }
    }
}
