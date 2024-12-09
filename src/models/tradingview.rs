use serde::Deserialize;

use super::TradeSignal;

/// `TradingViewAlert` is a struct that represents the payload data that TradingView sends to the server 
/// upon receiving an alert.
#[derive(Deserialize, Debug)]
pub struct TradingViewAlert {
    pub signal: TradeSignal,
    pub symbol: String,
    pub price: f64,
}