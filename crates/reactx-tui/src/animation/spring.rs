//! Spring physics for animated values.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use dioxus_core::prelude::use_hook;

use super::use_animation_frame;

/// A physically simulated value: stiffness, damping and mass.
///
/// The default is close to critically damped (no overshoot, settles in about
/// half a second). Lower the damping for a bouncy feel, or use
/// [`Spring::bouncy`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spring {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 170.0,
            damping: 26.0,
            mass: 1.0,
        }
    }
}

impl Spring {
    pub fn new(stiffness: f64, damping: f64) -> Self {
        Self {
            stiffness,
            damping,
            mass: 1.0,
        }
    }

    /// Overshoots the target a little before settling.
    pub fn bouncy() -> Self {
        Self::new(220.0, 12.0)
    }

    /// Snappier, stiffer movement.
    pub fn stiff() -> Self {
        Self::new(320.0, 30.0)
    }

    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass.max(f64::EPSILON);
        self
    }
}

const REST_POSITION: f64 = 1e-3;
const REST_VELOCITY: f64 = 1e-3;
const MAX_DT: f64 = 0.064;

struct State {
    value: f64,
    velocity: f64,
    config: Spring,
    last: Instant,
    settled: bool,
}

impl State {
    fn step(&mut self, target: f64, config: Spring, now: Instant) {
        self.config = config;
        let dt = (now - self.last).as_secs_f64().clamp(0.001, MAX_DT);
        self.last = now;
        let delta = self.value - target;
        if delta.abs() > REST_POSITION || self.velocity.abs() > REST_VELOCITY {
            self.settled = false;
        }
        if self.settled {
            self.value = target;
            self.velocity = 0.0;
            return;
        }
        let force = -config.stiffness * delta - config.damping * self.velocity;
        self.velocity += force / config.mass * dt;
        self.value += self.velocity * dt;
        if (self.value - target).abs() <= REST_POSITION && self.velocity.abs() <= REST_VELOCITY {
            self.value = target;
            self.velocity = 0.0;
            self.settled = true;
        }
    }
}

/// Animate a value towards `target` with spring physics.
///
/// ```ignore
/// let width = use_spring(target as f64, Spring::bouncy()) as u16;
/// ```
///
/// Unlike [`use_tween`](super::use_tween) the motion has momentum: when
/// `target` changes the current velocity carries over, so the value can
/// overshoot. The first render returns `target` unchanged; frames come from the
/// ~30 fps ticker and stop once the spring settles.
pub fn use_spring(target: f64, config: Spring) -> f64 {
    let state = use_hook(|| {
        Rc::new(RefCell::new(State {
            value: target,
            velocity: 0.0,
            config,
            last: Instant::now(),
            settled: true,
        }))
    });
    let tick = use_animation_frame();
    let now = Instant::now();
    let (value, moving) = {
        let mut state = state.borrow_mut();
        state.step(target, config, now);
        (state.value, !state.settled)
    };
    if moving {
        let _ = tick();
    }
    value
}
