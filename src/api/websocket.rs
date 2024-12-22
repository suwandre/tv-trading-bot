use std::sync::Arc;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc;
use serde_json::Value;

use crate::models::{ActiveTrade, AppState, BinanceTickerUpdate};

use crate::api::{close_paper_trade, is_trigger_hit};

/// Spawns:
/// 1) A task that connects to Binance WebSocket and sends price updates into an mpsc channel.
/// 2) A task that receives those price updates, checks active trades in memory, and closes them if triggered.
/// 
/// This keeps your `main` function minimal.
pub async fn start_price_listener(app_state: Arc<AppState>) {
    // create a channel for incoming price updates
    let (tx, mut rx) = mpsc::channel::<BinanceTickerUpdate>(100);

    // spawn the actual Binance WebSocket logic (which deserializes and sends ticker updates)
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        connect_and_subscribe_to_binance(tx_clone).await;
    });

    // spawn a task that receives the structured ticker updates
    let app_state_for_rx = app_state.clone();
    tokio::spawn(async move {
        while let Some(ticker_update) = rx.recv().await {
            println!("Received ticker update: {:?}", ticker_update);
            
            let symbol = &ticker_update.symbol;
            let price_str = &ticker_update.current_close_price;
            let price = price_str.parse::<f64>().unwrap_or(0.0);

            // check open trades for this symbol
            let trades_to_check: Vec<ActiveTrade> = {
                let map = app_state_for_rx.active_trades.lock().unwrap();
                map.values()
                   .filter(|trade| trade.pair.eq_ignore_ascii_case(symbol))
                   .cloned()
                   .collect()
            };

            for trade in trades_to_check {
                if is_trigger_hit(&trade, price) {
                    // if triggered, close it in DB, remove from in-memory, etc.
                    close_paper_trade(&app_state_for_rx, &trade.id, price).await;
                }
            }
        }
    });
}


/// Connects to Binance WebSocket and subscribes to one or multiple tickers.
/// Sends each incoming `ticker` event to the provided MPSC sender.
async fn connect_and_subscribe_to_binance(tx: mpsc::Sender<BinanceTickerUpdate>) {
    let binance_ws_url = "wss://stream.binance.com:9443/ws";
    let (ws_stream, _) = connect_async(binance_ws_url)
        .await
        .expect("Failed to connect to Binance WebSocket");

    println!("Connected to Binance WebSocket: {}", binance_ws_url);

    let (mut write, mut read) = ws_stream.split();

    // Subscribe message
    let subscription_message = serde_json::json!({
        "method": "SUBSCRIBE",
        "params": [
            "btcusdt@ticker"
        ],
        "id": 1
    });

    write
        .send(Message::Text(subscription_message.to_string().into()))
        .await
        .expect("Failed to send subscription message");

    println!("Subscribed to BTCUSDT ticker");

    // Continuously read messages
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(ticker_update) = serde_json::from_str::<BinanceTickerUpdate>(&text) {
                    let ticker_update_clone = ticker_update.clone();
                    // Forward to our channel
                    if tx.send(ticker_update_clone).await.is_err() {
                        eprintln!("(connect_and_subscribe_to_binance) Receiver dropped, stopping WebSocket connection.");
                        break;
                    }
                }
            }
            Ok(_) => { /* ignore non-text messages */ }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
}
