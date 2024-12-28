use std::sync::Arc;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc::{self, Receiver};
use serde_json::{from_str, json, Value};

use crate::constants::ACCEPTED_SYMBOLS;
use crate::models::{ActiveTrade, AppState, CoinbaseTickerUpdate, WsCommand};

use crate::api::{close_paper_trade, is_trigger_hit};

/// Connects to Coinbase WebSocket and subscribes to one or multiple tickers.
/// Sends each incoming `ticker` event to the provided MPSC sender.
pub async fn connect_and_subscribe_to_coinbase(tx: mpsc::Sender<CoinbaseTickerUpdate>) {
    let coinbase_ws_url = "wss://ws-feed.exchange.coinbase.com";
    let (ws_stream, _) = connect_async(coinbase_ws_url)
        .await
        .expect("(connect_and_subscribe_to_coinbase) Failed to connect to Coinbase WebSocket");

    println!("(connect_and_subscribe_to_coinbase) Connected to Coinbase: {}", coinbase_ws_url);

    let (mut write, mut read) = ws_stream.split();

    // subscribe to "ticker" for BTC-USD, ETH-USD (more will be added)
    let subscription_message = json!({
        "type": "subscribe",
        "product_ids": ["BTC-USD", "ETH-USD"],
        "channels": ["ticker"]
    });

    write
        .send(Message::Text(subscription_message.to_string().into()))
        .await
        .expect("(connect_and_subscribe_to_coinbase) Failed to send subscription message");

    println!("(connect_and_subscribe_to_coinbase) Subscribed to: [\"BTC-USD\", \"ETH-USD\"]");

    // continuously read messages
    while let Some(msg_result) = read.next().await {
        match msg_result {
            Ok(Message::Text(text)) => {
                // attempt to parse as `CoinbaseTickerUpdate`
                if let Ok(ticker_update) = from_str::<CoinbaseTickerUpdate>(&text) {
                    // we only want `type == "ticker"`
                    if ticker_update.update_type == "ticker" {
                        // send the typed struct to the receiver
                        if tx.send(ticker_update).await.is_err() {
                            eprintln!("(connect_and_subscribe_to_coinbase) Receiver dropped; stopping connection.");
                            break;
                        }
                    } else {
                        // e.g. "subscriptions" or something else
                        println!("(connect_and_subscribe_to_coinbase) Non-ticker message: {text}");
                    }
                }
            }
            Ok(_) => { /* ignore non-text/binary pings, etc. */ }
            Err(e) => {
                eprintln!("(connect_and_subscribe_to_coinbase) WebSocket error: {}", e);
                break;
            }
        }
    }

    println!("(connect_and_subscribe_to_coinbase) Exiting read loop.");
}

/// Spawns:
/// 1) A task that connects to Coinbase WebSocket and sends price updates into an mpsc channel.
/// 2) A task that receives those price updates, checks active trades in memory, and closes them if triggered.
pub async fn start_price_listener(app_state: Arc<AppState>) {
    // 1. Channel for typed ticker updates
    let (tx, mut rx) = mpsc::channel::<CoinbaseTickerUpdate>(100);

    // 2. Spawn the WebSocket subscription task
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        connect_and_subscribe_to_coinbase(tx_clone).await;
    });

    // 3. Spawn a consumer task
    let app_state_for_rx = app_state.clone();
    tokio::spawn(async move {
        while let Some(ticker_update) = rx.recv().await {
            // Print the entire struct for debugging
            println!("(start_price_listener) Received Coinbase update: {:?}", ticker_update);

            // Example: parse out the product_id and price
            let product_id = ticker_update.product_id.to_uppercase(); // "BTC-USD"
            let price_str = ticker_update.price.unwrap_or_else(|| "0.0".into());
            let price = price_str.parse::<f64>().unwrap_or(0.0);

            // Now find trades matching this product_id
            let trades_to_check: Vec<ActiveTrade> = {
                let map = app_state_for_rx.active_trades.lock().unwrap();
                map.values()
                    .filter(|trade| trade.pair.eq_ignore_ascii_case(&product_id))
                    .cloned()
                    .collect()
            };

            // For each trade, check if triggers are hit
            for trade in trades_to_check {
                if is_trigger_hit(&trade, price) {
                    println!("(start_price_listener) Trigger hit for trade: {:?}", trade);
                    
                    close_paper_trade(&app_state_for_rx, &trade.id, price).await;
                }
            }
        }
    });
}