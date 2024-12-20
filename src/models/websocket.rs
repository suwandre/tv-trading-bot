use serde::Deserialize;

/// Represents a ticker update from the Binance WebSocket.
/// 
/// Renaming is required because the data obtained from the WebSocket update is abbreviated.
#[derive(Debug, Deserialize)]
pub struct BinanceTickerUpdate {
    /// Event type (e.g., "24hrTicker").
    #[serde(rename = "e")]
    pub event_type: String,
    /// Event time in milliseconds since the Unix epoch.
    #[serde(rename = "E")]
    pub event_time: u64,
    /// Symbol (e.g., "BTCUSDT").
    #[serde(rename = "s")]
    pub symbol: String,
    /// Absolute price change over the last 24 hours.
    #[serde(rename = "p")]
    pub price_change: String,
    /// Percentage price change over the last 24 hours.
    #[serde(rename = "P")]
    pub price_change_percent: String,
    /// Weighted average price over the last 24 hours.
    #[serde(rename = "w")]
    pub weighted_avg_price: String,
    /// Previous day's closing price.
    #[serde(rename = "x")]
    pub prev_close_price: String,
    /// Current closing price.
    #[serde(rename = "c")]
    pub current_close_price: String,
    /// Quantity of the last trade executed.
    #[serde(rename = "Q")]
    pub last_trade_quantity: String,
    /// Best bid price.
    #[serde(rename = "b")]
    pub best_bid_price: String,
    /// Quantity available at the best bid price.
    #[serde(rename = "B")]
    pub best_bid_quantity: String,
    /// Best ask price.
    #[serde(rename = "a")]
    pub best_ask_price: String,
    /// Quantity available at the best ask price.
    #[serde(rename = "A")]
    pub best_ask_quantity: String,
    /// Opening price of the first trade in the last 24 hours.
    #[serde(rename = "o")]
    pub open_price: String,
    /// Highest price in the last 24 hours.
    #[serde(rename = "h")]
    pub high_price: String,
    /// Lowest price in the last 24 hours.
    #[serde(rename = "l")]
    pub low_price: String,
    /// Total traded base asset volume in the last 24 hours.
    #[serde(rename = "v")]
    pub base_asset_volume: String,
    /// Total traded quote asset volume in the last 24 hours.
    #[serde(rename = "q")]
    pub quote_asset_volume: String,
    /// Statistics open time in milliseconds since the Unix epoch.
    #[serde(rename = "O")]
    pub stats_open_time: u64,
    /// Statistics close time in milliseconds since the Unix epoch.
    #[serde(rename = "C")]
    pub stats_close_time: u64,
    /// Trade ID of the first trade in the last 24 hours.
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    /// Trade ID of the last trade in the last 24 hours.
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    /// Total number of trades executed in the last 24 hours.
    #[serde(rename = "n")]
    pub total_trades: u64,
}