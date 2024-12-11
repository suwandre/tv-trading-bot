use serde::{Deserialize, Serialize};

/// An instance of a executed trade, including the relevant details.
/// 
/// NOTE: Some fields will be null/empty upon initialization and will be filled once a trade is closed.
#[derive(Debug, Deserialize, Serialize)]
pub struct Trade<'a> {
    /// the unique database ID of the trade
    #[serde(rename = "_id")]
    pub id: &'a str,
    /// the status of the trade (active or closed)
    pub status: TradeStatus,
    /// the pair that the trade was executed on (e.g. SOL-USDT, ETH-BTC, etc.)
    pub pair: &'a str,
    /// the direction of the trade (long or short)
    pub direction: TradeDirection,
    /// quantity of the asset being traded
    pub quantity: f64,
}

/// Used to determine a buy or sell signal.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TradeSignal {
    Buy,
    Sell
}

/// Used to determine the direction of a trade.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TradeDirection {
    Long,
    Short
}

/// Used to determine the status of a trade.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TradeStatus {
    Active,
    Closed
}