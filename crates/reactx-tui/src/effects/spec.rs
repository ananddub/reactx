//! `animate*` attributes parsed into one-shot node effects.

use ratatui::style::Color;
use reactx_core::TuiColor;
use rustc_hash::FxHashMap;
use tachyonfx::fx::RepeatMode;
use tachyonfx::{Interpolation, Motion};

use super::parse::{
    parse_bool, parse_direction, parse_easing, parse_ms, parse_names, parse_repeat,
};

/// One animation requested by a node's attributes.
#[derive(Clone, Debug)]
pub(crate) struct Spec {
    pub(crate) names: Vec<String>,
    pub(crate) duration: u32,
    pub(crate) easing: Interpolation,
    pub(crate) direction: Motion,
    pub(crate) color: Color,
    pub(crate) gradient: u16,
    pub(crate) trigger: String,
    pub(crate) repeat: Option<RepeatMode>,
    pub(crate) ping_pong: bool,
    pub(crate) delay: u32,
}

impl Spec {
    /// The `animate` animation, played when the node appears or retriggers.
    pub(crate) fn parse(custom: &FxHashMap<String, String>) -> Option<Self> {
        Self::read(custom, "animate")
    }

    /// The `animate-exit` animation, played when the node leaves the tree.
    pub(crate) fn parse_exit(custom: &FxHashMap<String, String>) -> Option<Self> {
        Self::read(custom, "animate_exit")
    }

    fn read(custom: &FxHashMap<String, String>, attr: &str) -> Option<Self> {
        let names = parse_names(custom.get(attr)?);
        if names.is_empty() {
            return None;
        }
        let index = custom
            .get("animate_index")
            .and_then(|value| value.trim().parse::<u32>().ok())
            .unwrap_or(0);
        let stagger = custom
            .get("animate_stagger")
            .and_then(|value| parse_ms(value))
            .unwrap_or(100);
        let delay = custom
            .get("animate_delay")
            .and_then(|value| parse_ms(value))
            .unwrap_or(0)
            .saturating_add(index.saturating_mul(stagger));
        Some(Self {
            duration: custom
                .get("animate_duration")
                .and_then(|value| parse_ms(value))
                .unwrap_or(if attr == "animate_exit" { 400 } else { 600 }),
            easing: custom
                .get("animate_easing")
                .map(|value| parse_easing(value))
                .unwrap_or(Interpolation::QuadOut),
            direction: custom
                .get("animate_direction")
                .map(|value| parse_direction(value))
                .unwrap_or(Motion::LeftToRight),
            color: custom
                .get("animate_color")
                .and_then(|value| TuiColor::parse(value))
                .map(TuiColor::to_ratatui)
                .unwrap_or(Color::Black),
            gradient: custom
                .get("animate_gradient")
                .and_then(|value| value.trim().parse().ok())
                .unwrap_or(8),
            trigger: custom.get("animate_key").cloned().unwrap_or_default(),
            repeat: custom
                .get("animate_repeat")
                .and_then(|value| parse_repeat(value)),
            ping_pong: custom
                .get("animate_ping_pong")
                .is_some_and(|value| parse_bool(value)),
            delay,
            names,
        })
    }

    /// Identity of this animation; a change restarts it (used for `animate-key`).
    pub(crate) fn key(&self) -> String {
        format!(
            "{}|{}|{:?}|{:?}|{:?}|{}|{}|{:?}|{}|{}",
            self.names.join(","),
            self.duration,
            self.easing,
            self.direction,
            self.color,
            self.gradient,
            self.trigger,
            self.repeat,
            self.ping_pong,
            self.delay
        )
    }
}
