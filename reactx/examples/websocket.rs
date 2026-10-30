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
        SocketState::Connecting => jsx! { <span class="dim">Connecting...</span> },
        SocketState::Open => jsx! { <span class="dim">Connected, waiting for trades...</span> },
        SocketState::Message(trade) => {
            let headline = format!("{}  {}", trade.symbol, trade.price);
            let quantity = format!("Last quantity: {}", trade.quantity);
            jsx! {
                <span class="text-green-300 bold">{headline}</span>
                <span class="dim">{quantity}</span>
            }
        }
        SocketState::Closed(reason) => {
            let message = format!("Stream closed: {reason}");
            jsx! { <span class="text-yellow-300">{message}</span> }
        }
        SocketState::Failed(error) => {
            let message = format!("Stream failed: {error}");
            jsx! { <span class="text-rose-400">{message}</span> }
        }
    };

    jsx! {
        <ThemeProvider>
            <Box class="flex flex-col border border-blue-300 rounded p-1 gap-1">
                <span class="bold">BTC/USDT live trades</span>
                {stream}
                <span class="dim">Feed: Binance. Ctrl+C to quit.</span>
            </Box>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
