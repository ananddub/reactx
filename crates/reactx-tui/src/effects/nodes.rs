//! Per-node enter and exit animations requested through `animate*` attributes.

use std::time::{Duration, Instant};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use reactx_core::{NodeId, NodeTree};
use rustc_hash::{FxHashMap, FxHashSet};
use tachyonfx::{Effect, Shader};

use super::build::build;
use super::snapshot::Snapshot;
use super::spec::Spec;
use super::MAX_STEP;
use crate::layout;

mod ghost;
use ghost::Ghost;

struct NodeFx {
    key: String,
    area: Rect,
    effect: Effect,
}

/// A visible node with an `animate-exit` animation, tracked so its cells can
/// be captured for the exit ghost.
struct Watch {
    area: Rect,
    exit: Spec,
}

/// Effects attached to individual nodes, clipped to their visible rectangle.
#[derive(Default)]
pub(crate) struct NodeEffects {
    active: FxHashMap<NodeId, NodeFx>,
    /// Key played last, so finished animations do not restart on layout.
    played: FxHashMap<NodeId, String>,
    watched: FxHashMap<NodeId, Watch>,
    snapshots: FxHashMap<NodeId, Snapshot>,
    ghosts: Vec<Ghost>,
    last: Option<Instant>,
    cleanup: bool,
}

impl NodeEffects {
    /// Start, restart or drop node effects to match the current tree. Called
    /// after layout, before the frame is painted.
    pub(crate) fn sync(
        &mut self,
        tree: &NodeTree,
        layout: &FxHashMap<NodeId, Rect>,
        clips: &FxHashMap<NodeId, Rect>,
        offsets: &FxHashMap<NodeId, (i32, i32)>,
        area: Rect,
    ) {
        let mut live: FxHashSet<NodeId> = FxHashSet::default();
        let mut watched: FxHashMap<NodeId, Watch> = FxHashMap::default();
        for id in tree.nodes_in_order() {
            let Some(node) = tree.get(id) else { continue };
            if node.is_hidden() || node.is_placeholder() {
                continue;
            }
            let spec = Spec::parse(&node.attrs.custom);
            let exit = Spec::parse_exit(&node.attrs.custom);
            if spec.is_none() && exit.is_none() {
                continue;
            }
            let Some(rect) = layout.get(&id).copied() else {
                continue;
            };
            let (dx, dy) = offsets.get(&id).copied().unwrap_or((0, 0));
            let clip = clips.get(&id).copied().unwrap_or(area);
            let Some(visible) = layout::visible_rect(rect, dx, dy, clip) else {
                continue;
            };
            live.insert(id);
            if let Some(exit) = exit {
                watched.insert(
                    id,
                    Watch {
                        area: visible,
                        exit,
                    },
                );
            }
            let Some(spec) = spec else { continue };
            let key = spec.key();
            if self.active.get(&id).is_some_and(|fx| fx.key == key) {
                continue;
            }
            if self.played.get(&id).is_some_and(|played| *played == key) {
                continue;
            }
            let Some(effect) = build(&spec, visible) else {
                continue;
            };
            self.played.insert(id, key.clone());
            let fx = NodeFx {
                key,
                area: visible,
                effect,
            };
            self.active.insert(id, fx);
        }

        let gone: Vec<NodeId> = self
            .watched
            .keys()
            .copied()
            .filter(|id| !watched.contains_key(id))
            .collect();
        for id in gone {
            let (Some(watch), Some(snapshot)) =
                (self.watched.remove(&id), self.snapshots.remove(&id))
            else {
                continue;
            };
            let Some(effect) = build(&watch.exit, watch.area) else {
                continue;
            };
            self.ghosts.push(Ghost::new(snapshot, effect));
        }
        let still_watched: FxHashSet<NodeId> = watched.keys().copied().collect();
        self.watched = watched;
        self.snapshots.retain(|id, _| still_watched.contains(id));

        let stale: Vec<NodeId> = self
            .active
            .keys()
            .copied()
            .filter(|id| !live.contains(id))
            .collect();
        for id in stale {
            self.active.remove(&id);
            self.cleanup = true;
        }
        self.played.retain(|id, _| live.contains(id));
    }

    /// Advance active effects by one frame and drop the finished ones.
    pub(crate) fn process(&mut self, buf: &mut Buffer) {
        let Self {
            active,
            watched,
            snapshots,
            ghosts,
            last,
            cleanup,
            ..
        } = self;

        // Capture before the enter effects tint the freshly painted cells.
        for (id, watch) in watched.iter() {
            if let Some(snapshot) = Snapshot::capture(buf, watch.area) {
                snapshots.insert(*id, snapshot);
            }
        }
        if active.is_empty() && ghosts.is_empty() {
            *last = None;
            return;
        }
        let now = Instant::now();
        let step = last.map_or(Duration::ZERO, |last| now - last).min(MAX_STEP);
        *last = Some(now);

        let mut running = Vec::with_capacity(ghosts.len());
        for mut ghost in std::mem::take(ghosts) {
            if ghost.process(step, buf) {
                running.push(ghost);
            } else {
                *cleanup = true;
            }
        }
        *ghosts = running;

        let mut done = Vec::new();
        for (id, fx) in active.iter_mut() {
            fx.effect.process(step, buf, fx.area);
            if !fx.effect.running() {
                done.push(*id);
            }
        }
        if !done.is_empty() {
            for id in done {
                active.remove(&id);
            }
            *cleanup = true;
        }
    }

    pub(crate) fn is_active(&self) -> bool {
        !self.active.is_empty() || !self.ghosts.is_empty() || self.cleanup
    }

    pub(crate) fn take_cleanup(&mut self) -> bool {
        std::mem::take(&mut self.cleanup)
    }
}
