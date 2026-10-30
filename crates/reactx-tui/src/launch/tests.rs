//! Regression test for the launch loop's stream polling.

use std::collections::VecDeque;
use std::io;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use dioxus::prelude::*;
use futures_util::{Stream, StreamExt};
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::text::Line;
use ratatui::{Terminal, TerminalOptions, Viewport};
use reactx_core::static_out;
use tokio::time::timeout;

use super::run::flush_static;
use super::{handle_batch, Mode};
use crate::TuiRenderer;

fn buffer_text(buffer: &Buffer) -> String {
    buffer
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>()
}

#[test]
fn static_lines_follow_the_viewport_mode() {
    let _ = static_out::take_lines();

    // Inline mode writes queued lines above the live view.
    static_out::append_lines([Line::from("alpha"), Line::from("beta")]);
    let backend = TestBackend::new(20, 10);
    let options = TerminalOptions {
        viewport: Viewport::Inline(3),
    };
    let mut terminal = Terminal::with_options(backend, options).unwrap();
    flush_static(&mut terminal, Mode::Inline(3)).unwrap();
    assert!(static_out::take_lines().is_empty());
    let text = buffer_text(terminal.backend().buffer());
    assert!(text.contains("alpha"), "{text:?}");
    assert!(text.contains("beta"), "{text:?}");

    // Fullscreen has nowhere to scroll back to, so lines are dropped.
    static_out::append_lines([Line::from("gone")]);
    let mut fullscreen = Terminal::new(TestBackend::new(20, 10)).unwrap();
    flush_static(&mut fullscreen, Mode::Fullscreen).unwrap();
    assert!(static_out::take_lines().is_empty());
    let text = buffer_text(fullscreen.backend().buffer());
    assert!(!text.contains("gone"), "{text:?}");
}

fn app() -> Element {
    rsx! {
        div { "batch" }
    }
}

fn key(ch: char) -> CrosstermEvent {
    CrosstermEvent::Key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE))
}

#[derive(Default)]
struct State {
    queue: VecDeque<CrosstermEvent>,
    waker: Option<Waker>,
}

/// Terminal stream that keeps a single waker slot, like crossterm's
/// `EventStream`: a registration made by one poll is not replaced by later
/// polls until an event wakes it.
#[derive(Clone, Default)]
struct SharedStream {
    state: Arc<Mutex<State>>,
}

impl SharedStream {
    fn push(&self, event: CrosstermEvent) {
        let waker = {
            let mut state = self.state.lock().unwrap();
            state.queue.push_back(event);
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

impl Stream for SharedStream {
    type Item = io::Result<CrosstermEvent>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self.state.lock().unwrap();
        if let Some(event) = state.queue.pop_front() {
            return Poll::Ready(Some(Ok(event)));
        }
        if state.waker.is_none() {
            state.waker = Some(cx.waker().clone());
        }
        Poll::Pending
    }
}

#[tokio::test]
async fn batching_keeps_the_task_waker_registered() {
    let mut dom = VirtualDom::new(app);
    let mut renderer = TuiRenderer::new();
    dom.rebuild(&mut renderer);
    let mut stream = SharedStream::default();

    // One loop iteration: handle the first event plus the (empty) drain. The
    // drain's poll must register this task's waker, not a noop one, or the
    // next event would never wake the loop.
    assert!(handle_batch(key('a'), &mut stream, &dom, &mut renderer).await);

    let pusher = stream.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(25)).await;
        pusher.push(key('b'));
    });

    // The event must wake the loop; only the timeout deadline may end the
    // wait, so a noop-waker registration would show up as the full timeout.
    let start = Instant::now();
    let next = timeout(Duration::from_secs(2), stream.next()).await;
    assert!(
        matches!(next, Ok(Some(Ok(_)))),
        "the stream did not wake the event loop"
    );
    assert!(
        start.elapsed() < Duration::from_secs(1),
        "woke only on the timeout timer: {:?}",
        start.elapsed()
    );
}
