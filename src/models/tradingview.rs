use serde::Deserialize;

use super::TradeSignal;

/// `TradingViewAlert` is a struct that represents the payload data that TradingView sends to the server 
/// upon receiving an alert.
#[derive(Deserialize, Debug)]
pub struct TradingViewAlert {
    /// the alert name
    pub name: String,
    /// buy or sell
    pub signal: TradeSignal,
    /// the pair to execute the trade on (e.g. SOL-USDT, ETH-BTC, etc.)
    pub pair: String,
    /// the price of the base currency to the quote currency of the pair at the time of the alert
    /// (e.g. if the pair is 'SOL-USDT', then this price would be the price of 1 SOL in USDT)
    pub price: f64,
    /// the take profit price to set for the trade
    pub take_profit: Option<f64>,
    /// the stop loss price to set for the trade
    pub stop_loss: Option<f64>,
    /// the secret key to authenticate the trade execution request
    pub secret: String,
}