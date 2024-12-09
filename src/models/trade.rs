use serde::Deserialize;

/// Used to determine a buy or sell signal.
#[derive(Deserialize)]
pub enum TradeSignal {
    Buy,
    Sell
}