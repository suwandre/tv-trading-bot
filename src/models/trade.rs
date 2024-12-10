use serde::Deserialize;

/// Used to determine a buy or sell signal.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TradeSignal {
    Buy,
    Sell
}