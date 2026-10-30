//! Application lifecycle shared with components.

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

static EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);
static CLEAR_REQUESTED: AtomicBool = AtomicBool::new(false);

type Waiter = tokio::sync::oneshot::Sender<()>;

thread_local! {
    static RENDER_WAITERS: RefCell<Vec<Waiter>> = const { RefCell::new(Vec::new()) };
    static EXIT_WAITERS: RefCell<Vec<Waiter>> = const { RefCell::new(Vec::new()) };
}

/// Handle for app-wide actions, returned by [`use_app`].
#[derive(Clone, Copy, Debug, Default)]
pub struct AppHandle;

impl AppHandle {
    /// Stop the frame loop and restore the terminal.
    pub fn exit(&self) {
        EXIT_REQUESTED.store(true, Ordering::SeqCst);
    }

    /// Unmount the app, like `instance.unmount()` in Ink.
    pub fn unmount(&self) {
        self.exit();
    }

    /// Tear the app down and restore the terminal, like
    /// `instance.cleanup()` in Ink.
    pub fn cleanup(&self) {
        self.exit();
    }

    /// Clear the terminal before the next frame.
    pub fn clear(&self) {
        CLEAR_REQUESTED.store(true, Ordering::SeqCst);
    }

    /// Resolve once the next frame has been painted.
    pub async fn wait_until_render_flush(&self) {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        RENDER_WAITERS.with(|waiters| waiters.borrow_mut().push(sender));
        let _ = receiver.await;
    }

    /// Resolve when the app exits and the terminal is restored.
    pub async fn wait_until_exit(&self) {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        EXIT_WAITERS.with(|waiters| waiters.borrow_mut().push(sender));
        let _ = receiver.await;
    }
}

/// Access the running application, mirroring Ink's `useApp()`.
pub fn use_app() -> AppHandle {
    AppHandle
}

/// Whether a quit was requested since the last check.
pub(crate) fn take_exit() -> bool {
    EXIT_REQUESTED.swap(false, Ordering::SeqCst)
}

/// Whether the terminal should be cleared before the next frame.
pub(crate) fn take_clear() -> bool {
    CLEAR_REQUESTED.swap(false, Ordering::SeqCst)
}

/// Resolve every [`AppHandle::wait_until_render_flush`] waiter.
pub(crate) fn flush_render_waiters() {
    RENDER_WAITERS.with(|waiters| {
        for sender in waiters.borrow_mut().drain(..) {
            let _ = sender.send(());
        }
    });
}

/// Resolve every [`AppHandle::wait_until_exit`] waiter.
pub(crate) fn flush_exit_waiters() {
    EXIT_WAITERS.with(|waiters| {
        for sender in waiters.borrow_mut().drain(..) {
            let _ = sender.send(());
        }
    });
}

/// Clear flags and waiters left over from an earlier run.
pub(crate) fn reset() {
    EXIT_REQUESTED.store(false, Ordering::SeqCst);
    CLEAR_REQUESTED.store(false, Ordering::SeqCst);
    RENDER_WAITERS.with(|waiters| waiters.borrow_mut().clear());
    EXIT_WAITERS.with(|waiters| waiters.borrow_mut().clear());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_is_consumed_once() {
        AppHandle.exit();
        assert!(take_exit());
        assert!(!take_exit());
    }
}
