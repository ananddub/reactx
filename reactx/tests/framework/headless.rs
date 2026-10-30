//! Non-TTY fallback: launch renders headlessly and exits without a terminal.

use std::time::Duration;

use reactx::prelude::*;
use tokio::time::timeout;

fn exiting_app() -> Element {
    let app = use_app();
    use_hook(move || app.exit());
    rsx! { div { "headless" } }
}

#[tokio::test]
async fn headless_launch_finishes_without_a_terminal() {
    let options = LaunchOptions {
        interactive: Some(false),
        ..LaunchOptions::new()
    };
    timeout(Duration::from_secs(5), launch_with(exiting_app, options))
        .await
        .expect("headless launch should finish");
}
