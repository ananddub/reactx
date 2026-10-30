//! Runtime options shared by the launcher and the event loop.

use std::cell::RefCell;
use std::io::IsTerminal;
use std::sync::Arc;
use std::time::Duration;

/// Timing information passed to [`LaunchOptions::on_render`].
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderMetrics {
    /// Time spent painting the frame.
    pub render_time: Duration,
    /// Time since the previous frame.
    pub frame_time: Duration,
}

/// Callback invoked after every painted frame.
pub type RenderCallback = Arc<dyn Fn(RenderMetrics) + Send + Sync>;

/// Options for [`launch_with`](crate::launch_with).
///
/// Defaults match Ink: Ctrl+C quits, the kitty keyboard protocol is enabled,
/// and interactivity follows whether stdout is a terminal.
#[derive(Clone)]
pub struct LaunchOptions {
    /// Whether Ctrl+C exits the app. Disable to handle it yourself.
    pub exit_on_ctrl_c: bool,
    /// Push the kitty keyboard protocol for enhanced key reporting.
    pub kitty_keyboard: bool,
    /// Override screen-reader detection (`INK_SCREEN_READER` /
    /// `REACTX_SCREEN_READER` are consulted when `None`).
    pub screen_reader: Option<bool>,
    /// Cap frames at this rate. `None` renders whenever the app changes;
    /// the default of 30 matches Ink.
    pub max_fps: Option<f64>,
    /// Override TTY detection. When false, the app renders headlessly and
    /// prints the final frame on exit.
    pub interactive: Option<bool>,
    /// Called after each frame with paint timings.
    pub on_render: Option<RenderCallback>,
}

impl Default for LaunchOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl LaunchOptions {
    /// Options with every default made explicit.
    pub fn new() -> Self {
        Self {
            exit_on_ctrl_c: true,
            kitty_keyboard: true,
            screen_reader: None,
            max_fps: Some(30.0),
            interactive: None,
            on_render: None,
        }
    }
}

/// The resolved options the run loop uses.
#[derive(Clone)]
pub(crate) struct RuntimeConfig {
    pub exit_on_ctrl_c: bool,
    pub kitty_keyboard: bool,
    pub screen_reader: bool,
    pub max_fps: Option<f64>,
    pub interactive: bool,
    pub on_render: Option<RenderCallback>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            exit_on_ctrl_c: true,
            kitty_keyboard: true,
            screen_reader: false,
            max_fps: None,
            interactive: false,
            on_render: None,
        }
    }
}

impl RuntimeConfig {
    pub(crate) fn resolve(options: &LaunchOptions) -> Self {
        let screen_reader = options.screen_reader.unwrap_or_else(|| {
            ["REACTX_SCREEN_READER", "INK_SCREEN_READER"]
                .iter()
                .any(|key| std::env::var(key).is_ok_and(|value| value == "true"))
        });
        Self {
            exit_on_ctrl_c: options.exit_on_ctrl_c,
            kitty_keyboard: options.kitty_keyboard,
            screen_reader,
            max_fps: options.max_fps.filter(|fps| *fps > 0.0),
            interactive: options
                .interactive
                .unwrap_or_else(|| std::io::stdout().is_terminal()),
            on_render: options.on_render.clone(),
        }
    }

    /// Wall-clock interval between frames, when throttling is enabled.
    pub(crate) fn frame_interval(&self) -> Option<Duration> {
        self.max_fps
            .map(|fps| Duration::from_secs_f64(1.0 / fps.max(1.0)))
    }
}

thread_local! {
    static CONFIG: RefCell<Option<RuntimeConfig>> = const { RefCell::new(None) };
}

/// Install the options for the current run.
pub(crate) fn install(config: RuntimeConfig) {
    CONFIG.with(|slot| *slot.borrow_mut() = Some(config));
}

/// Remove installed options (called when a run ends).
pub(crate) fn clear() {
    CONFIG.with(|slot| *slot.borrow_mut() = None);
}

/// Read the active options, falling back to defaults outside a run loop.
pub(crate) fn with<R>(f: impl FnOnce(&RuntimeConfig) -> R) -> R {
    CONFIG.with(|slot| match slot.borrow().as_ref() {
        Some(config) => f(config),
        None => f(&RuntimeConfig::default()),
    })
}
