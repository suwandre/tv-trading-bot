use serde::Deserialize;

/// Used to determine a buy or sell signal.
#[derive(Deserialize, Debug, PartialEq)]
pub enum TradeSignal {
    Buy,
    Sell
}