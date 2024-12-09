use serde::Deserialize;

use super::trade::TradeSignal;

/// `TradingViewAlert` is a struct that represents the payload data that TradingView sends to the server 
/// upon receiving an alert.
#[derive(Deserialize)]
pub struct TradingViewAlert {
    pub signal: TradeSignal,
    pub symbol: String,
    pub price: f64,
}