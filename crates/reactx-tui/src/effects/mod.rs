//! Shader-style screen effects powered by [`tachyonfx`].
//!
//! Effects post-process the painted frame, so they work with any mix of
//! widgets: a `dissolve` scrambles the whole screen, `fade_from_fg` wakes it
//! up, `slide_in` pushes it in from an edge. Queue one from an event handler
//! with [`screen_effects`]; the renderer drains the queue, advances every
//! active effect with the frame delta and drops finished ones.
//!
//! Nodes opt into effects through `animate*` attributes: comma-separated
//! sequences, `animate-repeat`, `animate-ping-pong`, `animate-delay` and
//! `animate-index` staggering, plus `animate-exit`, which replays a removed
//! node's last painted cells while it animates away. Unknown names fall back to
//! shaders registered with [`register_shader`].

use std::cell::RefCell;
use std::time::{Duration, Instant};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::Frame;
use reactx_core::{NodeId, NodeTree};
use rustc_hash::FxHashMap;

pub use ratatui::style::Color;
pub use tachyonfx;
pub use tachyonfx::{fx, Effect, EffectTimer, Interpolation, Motion, Shader};

mod build;
mod custom;
mod nodes;
mod parse;
mod snapshot;
mod spec;

pub use custom::register_shader;
use nodes::NodeEffects;

/// Longest step handed to an effect, so a stalled loop does not jump to the
/// end of a transition.
const MAX_STEP: Duration = Duration::from_millis(100);

thread_local! {
    static PENDING: RefCell<Vec<Effect>> = const { RefCell::new(Vec::new()) };
}

/// Handle that queues screen effects. Cheap to copy; obtain it from
/// [`screen_effects`].
#[derive(Clone, Copy, Debug, Default)]
pub struct ScreenEffects;

/// Queue an effect to run over the whole screen from the next frame on.
pub fn screen_effects() -> ScreenEffects {
    ScreenEffects
}

impl ScreenEffects {
    /// Play `effect` over the full terminal area. Effects queued from inside a
    /// render run on the frame after that render.
    pub fn play(self, effect: impl Into<Effect>) {
        PENDING.with(|pending| pending.borrow_mut().push(effect.into()));
    }
}

/// Active effects owned by a [`TuiRenderer`](crate::TuiRenderer).
#[derive(Default)]
pub struct Effects {
    active: Vec<Effect>,
    nodes: NodeEffects,
    last: Option<Instant>,
    finished: bool,
}

impl Effects {
    /// Start an effect directly, without going through the thread queue.
    pub fn add(&mut self, effect: impl Into<Effect>) {
        self.active.push(effect.into());
    }

    /// Whether another frame is needed: an effect is running, one is queued,
    /// or one finished on the previous frame (so the screen can be repainted
    /// without it).
    pub fn is_active(&self) -> bool {
        self.finished
            || !self.active.is_empty()
            || PENDING.with(|pending| !pending.borrow().is_empty())
            || self.nodes.is_active()
    }

    /// Whether the back buffer should be reset before the next paint because
    /// an effect finished on the previous frame.
    pub fn take_cleanup(&mut self) -> bool {
        let cleanup = self.finished || self.nodes.take_cleanup();
        self.finished = false;
        cleanup
    }

    /// Reset the back buffer when the previous frame finished an effect.
    /// Called before painting the nodes.
    pub(crate) fn begin_frame(&mut self, frame: &mut Frame) {
        if self.take_cleanup() {
            frame.buffer_mut().reset();
        }
    }

    /// Advance screen and node effects over the freshly painted frame.
    pub(crate) fn end_frame(&mut self, frame: &mut Frame) {
        self.nodes.process(frame.buffer_mut());
        self.process(frame.area(), frame.buffer_mut());
    }

    /// Reconcile per-node animations with the freshly laid-out tree.
    pub(crate) fn sync_nodes(
        &mut self,
        tree: &NodeTree,
        layout: &FxHashMap<NodeId, Rect>,
        clips: &FxHashMap<NodeId, Rect>,
        offsets: &FxHashMap<NodeId, (i32, i32)>,
        area: Rect,
    ) {
        self.nodes.sync(tree, layout, clips, offsets, area);
    }

    /// Advance every effect by the time since the previous frame and drop the
    /// ones that finished. Called by the renderer after painting the nodes.
    pub fn process(&mut self, area: Rect, buf: &mut Buffer) {
        let now = Instant::now();
        let step = self
            .last
            .map_or(Duration::ZERO, |last| now - last)
            .min(MAX_STEP);
        self.last = Some(now);
        self.finished = false;

        PENDING.with(|pending| self.active.append(&mut pending.borrow_mut()));

        let before = self.active.len();
        self.active.retain_mut(|effect| {
            effect.process(step, buf, area);
            effect.running()
        });
        self.finished = self.active.len() != before;
    }
}

#[cfg(test)]
mod tests;
