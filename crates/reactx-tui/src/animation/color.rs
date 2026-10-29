//! Colour tweening between ratatui colours.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use dioxus_core::prelude::use_hook;
use ratatui::style::Color;

use super::{use_animation_frame, Tween};

/// The xterm palette for ANSI colours 0-15, as RGB components.
const ANSI: [[f64; 3]; 16] = [
    [0.0, 0.0, 0.0],
    [205.0, 0.0, 0.0],
    [0.0, 205.0, 0.0],
    [205.0, 205.0, 0.0],
    [0.0, 0.0, 238.0],
    [205.0, 0.0, 205.0],
    [0.0, 205.0, 205.0],
    [229.0, 229.0, 229.0],
    [127.0, 127.0, 127.0],
    [255.0, 0.0, 0.0],
    [0.0, 255.0, 0.0],
    [255.0, 255.0, 0.0],
    [92.0, 92.0, 255.0],
    [255.0, 0.0, 255.0],
    [0.0, 255.0, 255.0],
    [255.0, 255.0, 255.0],
];

struct State {
    from: [f64; 3],
    to: [f64; 3],
    start: Instant,
    config: Tween,
    running: bool,
}

impl State {
    fn retarget(&mut self, target: [f64; 3], config: Tween, now: Instant) {
        self.from = self.sample(now);
        self.to = target;
        self.start = now;
        self.config = config;
        self.running = self.from != self.to;
    }

    fn sample(&self, now: Instant) -> [f64; 3] {
        if !self.running {
            return self.to;
        }
        let total = self.config.duration.as_secs_f64();
        if total <= 0.0 {
            return self.to;
        }
        let t = ((now - self.start).as_secs_f64() / total).clamp(0.0, 1.0);
        let alpha = f64::from(self.config.easing.alpha(t as f32));
        std::array::from_fn(|i| self.from[i] + (self.to[i] - self.from[i]) * alpha)
    }

    fn settled(&self, now: Instant) -> bool {
        !self.running || self.config.duration.is_zero() || now - self.start >= self.config.duration
    }
}

/// Animate a [`Color`] towards `target`, rerendering while it blends.
///
/// ```ignore
/// let accent = use_color_tween(theme_accent(), 300u32);
/// ```
///
/// `config` accepts the same values as [`use_tween`](super::use_tween)
/// ([`Tween`], a `Duration` or a millisecond `u32`). Named and indexed colours
/// are mapped to their xterm RGB values while blending. Colours without an RGB
/// equivalent (`Color::Reset`) are returned unchanged, and once the blend
/// finishes the exact `target` is returned.
pub fn use_color_tween(target: Color, config: impl Into<Tween>) -> Color {
    let config = config.into();
    let state = use_hook(|| {
        let channels = rgb(target).unwrap_or([0.0; 3]);
        Rc::new(RefCell::new(State {
            from: channels,
            to: channels,
            start: Instant::now(),
            config,
            running: false,
        }))
    });
    let tick = use_animation_frame();
    let Some(channels) = rgb(target) else {
        return target;
    };
    let now = Instant::now();
    let (value, running) = {
        let mut state = state.borrow_mut();
        if state.to != channels || state.config != config {
            state.retarget(channels, config, now);
        }
        if state.settled(now) {
            state.running = false;
        }
        (state.sample(now), state.running)
    };
    if running {
        let _ = tick();
        return to_color(value);
    }
    target
}

fn to_color(channels: [f64; 3]) -> Color {
    let [r, g, b] = channels.map(|channel| channel.round().clamp(0.0, 255.0) as u8);
    Color::Rgb(r, g, b)
}

fn rgb(color: Color) -> Option<[f64; 3]> {
    let index = match color {
        Color::Black => 0,
        Color::Red => 1,
        Color::Green => 2,
        Color::Yellow => 3,
        Color::Blue => 4,
        Color::Magenta => 5,
        Color::Cyan => 6,
        Color::Gray => 7,
        Color::DarkGray => 8,
        Color::LightRed => 9,
        Color::LightGreen => 10,
        Color::LightYellow => 11,
        Color::LightBlue => 12,
        Color::LightMagenta => 13,
        Color::LightCyan => 14,
        Color::White => 15,
        Color::Rgb(r, g, b) => return Some([f64::from(r), f64::from(g), f64::from(b)]),
        Color::Indexed(index) => return Some(indexed(index)),
        _ => return None,
    };
    Some(ANSI[index])
}

fn indexed(index: u8) -> [f64; 3] {
    if index < 16 {
        return ANSI[index as usize];
    }
    if index < 232 {
        const STEPS: [f64; 6] = [0.0, 95.0, 135.0, 175.0, 215.0, 255.0];
        let n = u32::from(index - 16);
        return [
            STEPS[(n / 36) as usize],
            STEPS[((n / 6) % 6) as usize],
            STEPS[(n % 6) as usize],
        ];
    }
    [8.0 + f64::from(index - 232) * 10.0; 3]
}
