//! Exit-animation ghosts replayed from captured cells.

use std::time::Duration;

use ratatui::buffer::Buffer;
use tachyonfx::{Effect, Shader};

use super::super::snapshot::Snapshot;

/// A node that left the tree, replayed from its last painted cells while its
/// `animate-exit` effect runs.
pub(super) struct Ghost {
    snapshot: Snapshot,
    effect: Effect,
}

impl Ghost {
    pub(super) fn new(snapshot: Snapshot, effect: Effect) -> Self {
        Self { snapshot, effect }
    }

    /// Restore the captured cells and advance the exit effect by `step`.
    /// Returns `false` once the effect has finished.
    pub(super) fn process(&mut self, step: Duration, buf: &mut Buffer) -> bool {
        let Some(area) = self.snapshot.restore(buf) else {
            return false;
        };
        self.effect.process(step, buf, area);
        self.effect.running()
    }
}
