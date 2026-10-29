//! Value tweens driven by the animation ticker.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use dioxus_core::prelude::use_hook;

use super::use_animation_frame;
use crate::effects::Interpolation;

/// Duration and easing used by [`use_tween`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tween {
    pub duration: Duration,
    pub easing: Interpolation,
}

impl Tween {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            easing: Interpolation::QuadOut,
        }
    }

    pub fn ease(mut self, easing: Interpolation) -> Self {
        self.easing = easing;
        self
    }
}

impl From<Duration> for Tween {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

/// Milliseconds.
impl From<u32> for Tween {
    fn from(millis: u32) -> Self {
        Self::new(Duration::from_millis(u64::from(millis)))
    }
}

struct State {
    from: f64,
    to: f64,
    start: Instant,
    config: Tween,
    running: bool,
}

impl State {
    fn retarget(&mut self, target: f64, config: Tween, now: Instant) {
        self.from = self.sample(now);
        self.to = target;
        self.start = now;
        self.config = config;
        self.running = self.from != self.to;
    }

    fn sample(&self, now: Instant) -> f64 {
        if !self.running {
            return self.to;
        }
        let total = self.config.duration.as_secs_f64();
        if total <= 0.0 {
            return self.to;
        }
        let t = ((now - self.start).as_secs_f64() / total).clamp(0.0, 1.0);
        let eased = f64::from(self.config.easing.alpha(t as f32));
        self.from + (self.to - self.from) * eased
    }

    fn settled(&self, now: Instant) -> bool {
        !self.running || self.config.duration.is_zero() || now - self.start >= self.config.duration
    }
}

/// Animate a value towards `target`, rerendering while it is still moving.
///
/// `config` accepts a [`Tween`], a `Duration` or a millisecond `u32`:
///
/// ```ignore
/// let width = use_tween(target_width(), 300u32);
/// let value = use_tween(target(), Tween::new(Duration::from_millis(250))
///     .ease(Interpolation::SineOut));
/// ```
///
/// The first render returns `target` unchanged; the tween starts whenever
/// `target` changes afterwards. Frames come from the same ~30 fps ticker as
/// [`use_animation_frame`], and stop once the value settles.
pub fn use_tween(target: f64, config: impl Into<Tween>) -> f64 {
    let config = config.into();
    let state = use_hook(|| {
        Rc::new(RefCell::new(State {
            from: target,
            to: target,
            start: Instant::now(),
            config,
            running: false,
        }))
    });
    let tick = use_animation_frame();
    let now = Instant::now();

    let (value, running) = {
        let mut state = state.borrow_mut();
        if state.to != target || state.config != config {
            state.retarget(target, config, now);
        }
        if state.settled(now) {
            state.running = false;
        }
        (state.sample(now), state.running)
    };
    if running {
        let _ = tick();
    }
    value
}
