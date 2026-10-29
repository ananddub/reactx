//! Stream live trades from a public JSON WebSocket, written with HTML-like
//! `jsx!` syntax.
//!
//! Run with: `cargo run -p reactx --example websocket`
//! Needs network access to `wss://stream.binance.com:9443`.

#![allow(non_snake_case)]

use reactx::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct Trade {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
}

#[component]
fn App() -> Element {
    let state = use_websocket::<Trade>("wss://stream.binance.com:9443/ws/btcusdt@trade");

    let stream = match &*state.read() {
        SocketState::Connecting => jsx! { <Text dim>Connecting...</Text> },
        SocketState::Open => jsx! { <Text dim>Connected, waiting for trades...</Text> },
        SocketState::Message(trade) => {
            let headline = format!("{}  {}", trade.symbol, trade.price);
            let quantity = format!("Last quantity: {}", trade.quantity);
            jsx! {
                <Text color="#a6e3a1" bold>{headline}</Text>
                <Text dim>{quantity}</Text>
            }
        }
        SocketState::Closed(reason) => {
            let message = format!("Stream closed: {reason}");
            jsx! { <Text color="#f9e2af">{message}</Text> }
        }
        SocketState::Failed(error) => {
            let message = format!("Stream failed: {error}");
            jsx! { <Text color="#f38ba8">{message}</Text> }
        }
    };

    jsx! {
        <ThemeProvider>
            <Container border="rounded" border-color="#89b4fa" padding="1" gap={Some(1)}>
                <Text bold>BTC/USDT live trades</Text>
                {stream}
                <Text dim>Feed: Binance. Ctrl+C to quit.</Text>
            </Container>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
