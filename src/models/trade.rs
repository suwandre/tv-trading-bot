use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An instance of a executed trade, including the relevant details.
/// 
/// NOTE: Some fields will be null/empty upon initialization and will be filled once a trade is closed.
#[derive(Debug, Deserialize, Serialize)]
pub struct Trade<'a> {
    /// the unique database ID of the trade.
    #[serde(rename = "_id")]
    pub id: &'a str,
    /// the pair that the trade was executed on (e.g. SOL-USDT, ETH-BTC, etc.)
    pub pair: &'a str,
    /// the direction of the trade (long or short)
    pub direction: TradeDirection,
    /// the kind of trade (paper or live)
    pub kind: TradeKind,
    /// the status of the trade (active or closed)
    pub status: TradeStatus,
    /// quantity of the base currency of the pair being traded.
    /// 
    /// (e.g. if SOL-USDT, then this would be the quantity of SOL)
    pub quantity: f64,
    /// the price of the base currency to the quote currency of the pair at the time of the trade.
    /// 
    /// (e.g. if the pair is 'SOL-USDT', then this price would be the price of 1 SOL in USDT)
    pub entry_price: f64,
    /// the price of the base currency to the quote currency of the pair at the time of closing the trade.
    pub exit_price: Option<f64>,
    /// the leverage used for the trade.
    pub leverage: TradeLeverage,
    /// the timestamp of when the trade was opened.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub open_timestamp: DateTime<Utc>,
    /// the timestamp of when the trade was closed.
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub close_timestamp: Option<DateTime<Utc>>,
    /// the profit or loss of the trade (in USDT value).
    /// 
    /// this will already take the base profit/loss and all fees into account.
    pub pnl: Option<f64>,
    /// the fees paid for closing and opening the trade (in USDT value).
    pub execution_fees: f64,
    /// the funding fees paid for holding the trade over several hours or days (in USDT value).
    pub funding_fees: Option<f64>,

}

/// Used to determine a buy or sell signal.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TradeSignal {
    Buy,
    Sell
}

/// Used to determine the kind of trade (paper or live).
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TradeKind {
    Paper,
    Live
}

/// Used to determine the direction of a trade.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TradeDirection {
    Long,
    Short
}

/// Used to determine the status of a trade.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TradeStatus {
    Active,
    Closed
}

/// Used to determine the leverage of a trade.
#[derive(Serialize, Deserialize, Debug)]
pub enum TradeLeverage {
    #[serde(rename = "1x")]
    One,
    #[serde(rename = "2x")]
    Two,
    #[serde(rename = "3x")]
    Three,
    #[serde(rename = "5x")]
    Five,
    #[serde(rename = "10x")]
    Ten
}

