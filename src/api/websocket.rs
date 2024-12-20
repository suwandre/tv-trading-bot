use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc;
use serde_json::Value;

/// Starts a WebSocket connection to Binance and sends ticker updates through the provided channel.
pub async fn start_binance_websocket(tx: mpsc::Sender<Value>) {
    let binance_ws_url = "wss://stream.binance.com:9443/ws";

    // Connect to Binance WebSocket
    let (ws_stream, _) = connect_async(binance_ws_url)
        .await
        .expect("(start_binance_websocket) Failed to connect to Binance WebSocket");

    println!("(start_binance_websocket) Connected to Binance WebSocket");

    // Split the stream into separate read and write halves
    let (mut write, mut read) = ws_stream.split();

    // Subscribe to ticker updates (e.g., BTCUSDT)
    let subscription_message = serde_json::json!({
        "method": "SUBSCRIBE",
        "params": [
            "btcusdt@ticker" // Add more symbols if needed
        ],
        "id": 1
    });

    write
        .send(Message::Text(subscription_message.to_string().into()))
        .await
        .expect("(start_binance_websocket) Failed to send subscription message");

    println!("(start_binance_websocket) Subscribed to BTCUSDT ticker updates");

    // Receive and send messages through the channel
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                if let Ok(json_msg) = serde_json::from_str::<Value>(&text) {
                    if tx.send(json_msg).await.is_err() {
                        println!("(start_binance_websocket) Receiver dropped; stopping WebSocket connection");
                        break;
                    }
                } else {
                    println!("(start_binance_websocket) Failed to parse message: {}", text);
                }
            }
            Ok(_) => {} // Ignore non-text messages
            Err(e) => {
                eprintln!("(start_binance_websocket) WebSocket error: {}", e);
                break;
            }
        }
    }
}
