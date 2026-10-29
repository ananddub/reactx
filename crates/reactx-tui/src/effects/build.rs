//! Turn an animation [`Spec`] into a tachyonfx effect for one rectangle.

use ratatui::layout::Rect;
use tachyonfx::{fx, Effect, IntoEffect};

use super::custom::custom_effect;
use super::spec::Spec;

/// Build the composed effect: sequence, ping-pong, repeat and delay wrappers
/// in that order, clipped to `area`. `None` when a name is unknown and not
/// registered through [`register_shader`](super::register_shader).
pub(crate) fn build(spec: &Spec, area: Rect) -> Option<Effect> {
    let mut effect = if let [name] = spec.names.as_slice() {
        single(name, spec, area)?
    } else {
        let parts: Option<Vec<Effect>> = spec
            .names
            .iter()
            .map(|name| single(name, spec, area))
            .collect();
        fx::sequence(&parts?)
    };
    if spec.ping_pong {
        effect = fx::ping_pong(effect);
    }
    if let Some(mode) = spec.repeat {
        effect = fx::repeat(effect, mode);
    }
    if spec.delay > 0 {
        effect = fx::delay(spec.delay, effect);
    }
    Some(effect.with_area(area))
}

fn single(name: &str, spec: &Spec, area: Rect) -> Option<Effect> {
    let timer = (spec.duration, spec.easing);
    Some(match name {
        "fade" | "fade-in" | "fade_in" => fx::fade_from_fg(spec.color, timer),
        "fade-out" | "fade_out" | "fade-to" => fx::fade_to_fg(spec.color, timer),
        "dissolve" => fx::dissolve(timer),
        "coalesce" => fx::coalesce(timer),
        "slide-in" | "slide_in" => {
            fx::slide_in(spec.direction, spec.gradient, 0, spec.color, timer)
        }
        "slide-out" | "slide_out" => {
            fx::slide_out(spec.direction, spec.gradient, 0, spec.color, timer)
        }
        "sweep-in" | "sweep_in" => {
            fx::sweep_in(spec.direction, spec.gradient, 0, spec.color, timer)
        }
        "sweep-out" | "sweep_out" => {
            fx::sweep_out(spec.direction, spec.gradient, 0, spec.color, timer)
        }
        "explode" => fx::explode(8.0, 0.2, timer),
        "hsl-shift" | "hsl_shift" => fx::hsl_shift_fg([120.0, 20.0, 0.0], timer),
        "glitch" => fx::Glitch::builder()
            .cell_glitch_ratio(0.35)
            .action_start_delay_ms(0..120)
            .action_ms(80..240)
            .build()
            .into_effect(),
        other => custom_effect(other, area)?,
    })
}
