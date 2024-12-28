use serde::Deserialize;

/// The commands that are sent to the writer task.
/// 
/// Used to subscribe and unsubscribe from the WebSocket to fetch/unfetch tickers.
#[derive(Debug)]
pub enum WsCommand {
    Subscribe(String),
    Unsubscribe(String),
}

/// Represents a ticker update from Coinbase WebSocket.
#[derive(Debug, Deserialize, Clone)]
pub struct CoinbaseTickerUpdate {
    #[serde(rename = "type")]
    pub update_type: String, // typically "ticker"

    pub sequence: Option<u64>,
    pub product_id: String,  // "BTC-USD"
    
    pub price: Option<String>,       // "96289.34"
    pub open_24h: Option<String>,    // "95393.55"
    pub volume_24h: Option<String>,  // "11941.80121342"
    pub low_24h: Option<String>,
    pub high_24h: Option<String>,
    pub volume_30d: Option<String>,
    pub best_bid: Option<String>,
    pub best_bid_size: Option<String>,
    pub best_ask: Option<String>,
    pub best_ask_size: Option<String>,
    pub side: Option<String>,        // "buy" or "sell" in coinbase feed
    pub time: Option<String>,        // "2024-12-27T10:50:33.372945Z"
    pub trade_id: Option<u64>,
    pub last_size: Option<String>,
}