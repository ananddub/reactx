//! Timer-driven rerenders for animation.

use std::time::Duration;

use dioxus_core::prelude::{spawn, use_drop, use_hook};
use dioxus_signals::{Signal, Writable};

mod color;
mod spring;
mod tween;

pub use color::use_color_tween;
pub use spring::{use_spring, Spring};
pub use tween::{use_tween, Tween};

/// Interval between animation frames (~30 fps).
const FRAME: Duration = Duration::from_millis(33);

/// A frame counter that advances about 30 times per second.
///
/// Reading the signal inside a component subscribes it to ticks, so the
/// component rerenders every frame: spinner glyphs, gauge values, canvas
/// shapes and marquee offsets can all be derived from the counter. The ticker
/// stops when the component unmounts.
///
/// Ticks run on the tokio runtime that [`launch`](crate::launch) sets up.
pub fn use_animation_frame() -> Signal<u64> {
    use_ticker(FRAME)
}

/// Like [`use_animation_frame`] but with a custom frame interval.
pub fn use_ticker(period: Duration) -> Signal<u64> {
    let frame = use_hook(|| Signal::new(0u64));
    let task = use_hook(move || {
        spawn(async move {
            let mut frame = frame;
            let mut ticker = tokio::time::interval(period);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            ticker.tick().await;
            let mut value = 0u64;
            loop {
                ticker.tick().await;
                value += 1;
                frame.set(value);
            }
        })
    });
    use_drop(move || task.cancel());
    frame
}

#[cfg(test)]
mod tests;
