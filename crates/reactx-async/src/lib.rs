//! Async data primitives: JSON fetching, WebSockets and suspense.

use dioxus::prelude::*;
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use tokio_tungstenite::tungstenite::Message;

/// Fetch JSON from `url` once, rerendering when the request settles.
///
/// Read the result through [`Resource`]'s `read()`/`value()`:
///
/// ```no_run
/// # use reactx_async::use_fetch;
/// # use serde::Deserialize;
/// # #[derive(Deserialize)] struct Weather { temp: f32 }
/// let weather = use_fetch::<Weather>("https://example.com/weather");
/// ```
pub fn use_fetch<T>(url: impl ToString) -> Resource<Result<T, String>>
where
    T: DeserializeOwned + 'static,
{
    let url = url.to_string();
    use_resource(move || {
        let url = url.clone();
        async move { fetch_json::<T>(&url).await }
    })
}

async fn fetch_json<T: DeserializeOwned>(url: &str) -> Result<T, String> {
    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .map_err(|error| error.to_string())?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!("HTTP {status}"));
    }
    response
        .json::<T>()
        .await
        .map_err(|error| error.to_string())
}

/// Connection progress for [`use_websocket`].
#[derive(Debug, Clone, PartialEq)]
pub enum SocketState<T> {
    Connecting,
    Open,
    Message(T),
    Closed(String),
    Failed(String),
}

impl<T> SocketState<T> {
    pub fn message(&self) -> Option<&T> {
        match self {
            Self::Message(value) => Some(value),
            _ => None,
        }
    }
}

/// Connect to a JSON WebSocket, exposing the latest decoded message.
///
/// Messages are expected to be JSON that deserializes into `T`.
pub fn use_websocket<T>(url: impl ToString) -> Signal<SocketState<T>>
where
    T: DeserializeOwned + PartialEq + 'static,
{
    let url = url.to_string();
    use_hook(move || {
        let state = Signal::new(SocketState::Connecting);
        spawn(async move {
            let mut state = state;
            let result = drive_socket(url, state).await;
            if let Err(error) = result {
                state.set(SocketState::Failed(error));
            }
        });
        state
    })
}

async fn drive_socket<T>(url: String, mut state: Signal<SocketState<T>>) -> Result<(), String>
where
    T: DeserializeOwned + PartialEq + 'static,
{
    let (stream, _) = tokio_tungstenite::connect_async(&url)
        .await
        .map_err(|error| error.to_string())?;
    state.set(SocketState::Open);

    let mut stream = stream;
    while let Some(message) = stream.next().await {
        match message.map_err(|error| error.to_string())? {
            Message::Text(text) => {
                let value =
                    serde_json::from_str::<T>(text.as_str()).map_err(|error| error.to_string())?;
                state.set(SocketState::Message(value));
            }
            Message::Binary(bytes) => {
                let value =
                    serde_json::from_slice::<T>(&bytes).map_err(|error| error.to_string())?;
                state.set(SocketState::Message(value));
            }
            Message::Close(_) => {
                state.set(SocketState::Closed("closed by peer".to_string()));
                return Ok(());
            }
            _ => {}
        }
    }

    state.set(SocketState::Closed("stream ended".to_string()));
    Ok(())
}

/// Suspense boundary with a terminal-friendly default fallback.
#[component]
pub fn Suspense(children: Element, #[props(default)] fallback: Option<Element>) -> Element {
    let fallback = fallback.unwrap_or_else(|| rsx! { span { style: "dim:true", "Loading…" } });
    rsx! {
        SuspenseBoundary {
            fallback: move |_| rsx! { {fallback.clone()} },
            children: children,
        }
    }
}
