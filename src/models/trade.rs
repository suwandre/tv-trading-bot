use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// A trade instance that is generated upon executing a trade.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveTrade {
    /// the unique database ID of the trade.
    #[serde(rename = "_id")]
    pub id: ObjectId,
    /// the pair that the trade was executed on (e.g. SOL-USDT, ETH-BTC, etc.)
    pub pair: String,
    /// the direction of the trade (long or short)
    pub direction: TradeDirection,
    /// the kind of trade (paper or live)
    pub kind: TradeKind,
    /// the timestamp of when the trade was opened.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub open_timestamp: DateTime<Utc>,
    /// quantity of the base currency of the pair being traded.
    /// 
    /// (e.g. if SOL-USDT, then this would be the quantity of SOL)
    pub quantity: f64,
    /// the price of the base currency to the quote currency of the pair at the time of the trade.
    /// 
    /// (e.g. if the pair is 'SOL-USDT', then this price would be the price of 1 SOL in USDT)
    pub entry_price: f64,
    /// the leverage used for the trade.
    /// 
    /// if spot trading, this will be set to 1x.
    pub leverage: TradeLeverage,
    /// the liquidation price of the trade.
    pub liquidation_price: f64,
    /// if a take profit (TP) price is set, it will be stored here.
    pub take_profit: Option<f64>,
    /// if a stop loss (SL) price is set, it will be stored here.
    pub stop_loss: Option<f64>,
}

/// An instance of a trade that has been successfully closed.
/// 
/// This will include all the relevant details of the trade, including the profit/loss, fees, etc.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedTrade {
    /// the unique database ID of the trade.
    #[serde(rename = "_id")]
    pub id: ObjectId,
    /// the pair that the trade was executed on (e.g. SOL-USDT, ETH-BTC, etc.)
    pub pair: String,
    /// the direction of the trade (long or short)
    pub direction: TradeDirection,
    /// the kind of trade (paper or live)
    pub kind: TradeKind,
    /// quantity of the base currency of the pair that was traded.
    /// 
    /// (e.g. if SOL-USDT, then this would be the quantity of SOL)
    pub quantity: f64,
    /// the price of the base currency to the quote currency of the pair when the trade was opened/executed.
    /// 
    /// (e.g. if the pair is 'SOL-USDT', then this price would be the price of 1 SOL in USDT)
    pub entry_price: f64,
    /// the price of the base currency to the quote currency of the pair at the time of closing the trade.
    pub exit_price: f64,
    /// the leverage used for the trade.
    /// 
    /// if spot trading, this will be set to 1x.
    pub leverage: TradeLeverage,
    /// the liquidation price of the trade.
    pub liquidation_price: f64,
    /// the timestamp of when the trade was opened.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub open_timestamp: DateTime<Utc>,
    /// the timestamp of when the trade was closed.
    #[serde(with = "chrono::serde::ts_seconds")]
    pub close_timestamp: DateTime<Utc>,
    /// the profit or loss of the trade (in USDT value).
    /// 
    /// this will already take the base profit/loss and all fees into account.
    pub pnl: f64,
    /// the return on equity (ROE) of the trade (in percentage format).
    /// 
    /// this takes leverage into account.
    pub roe: f64,
    /// the fees paid for closing and opening the trade (in USDT value). used primarily in paper trades only, unless the exchange
    /// that the trade was executed in provides this value (for live trades).
    pub execution_fees: f64,
    /// the funding fees paid for holding the trade over several hours or days (in USDT value). used primarily in paper trades only, unless the exchange
    /// the trade was executed in provides this value (for live trades).
    /// 
    /// at the start of trades, all `funding_fees` will start at 0 and accumulate after 1, 4 or 8 hours depending on the exchange.
    /// 
    /// for spot trades, this will be kept at 0.
    pub funding_fees: f64,
}

impl From<TradeSignal> for TradeDirection {
    /// Explicitly converts a `TradeSignal` into a `TradeDirection`.
    /// 
    /// Mainly used when trying to check an existing trade's direction to the alert's signal to determine whether to close a trade or let it be.
    /// 
    /// For instance, if the existing trade's direction is `Long` and the alert's signal is `Buy`, then do nothing. 
    /// However, if the alert's signal is `Sell`, then close the trade. This also works vice versa.
    fn from(signal: TradeSignal) -> Self {
        match signal {
            TradeSignal::Buy => TradeDirection::Long,
            TradeSignal::Sell => TradeDirection::Short
        }
    }
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
#[serde(rename_all = "camelCase")]
pub enum TradeKind {
    Paper,
    Live
}

/// Used to determine the direction of a trade.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

