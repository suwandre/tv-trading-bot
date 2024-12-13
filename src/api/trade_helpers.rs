use chrono::DateTime;

use crate::{constants::{EXECUTION_FEE_PERCENTAGE, MAINTENANCE_MARGIN}, models::TradeDirection};

/// Calculate the liquidation price of a trade based on the entry price, leverage, and direction. Used for both long and short trades.
/// 
/// Only used in paper trading to simulate real liquidation prices.
/// 
/// Maintenance margin is also taken into account.
pub fn calc_liquidation_price(
    entry_price: f64,
    leverage: f64,
    direction: TradeDirection
) -> f64 {
    if direction == TradeDirection::Long {
        // liq price = entry price * (1 - (1 / leverage) + (maintenance margin [in ratio format] / leverage))
        entry_price * (1.0 - (1.0 / leverage) + ((MAINTENANCE_MARGIN / 100.0) / leverage)) 
    } else {
        // liq price = entry price * (1 + (1 / leverage) - (maintenance margin [in ratio format] / leverage))
        entry_price * (1.0 + (1.0 / leverage) - ((MAINTENANCE_MARGIN / 100.0) / leverage))
    }
}

/// Calculate the final execution fee for a trade, taking both opening and closing fees into account.
/// 
/// Used purely for paper trading only.
pub fn calc_final_execution_fees(quantity: f64, entry_price: f64) -> f64 {
    2.0 * (EXECUTION_FEE_PERCENTAGE / 100.0 * quantity * entry_price)
}

// pub fn calc_final_funding_fees(
//     open_timestamp: DateTime<Utc>, 
//     close_timestamp: DateTime<Utc>,
//     // the current margin value of the position. calculated by the current price * quantity.
//     notional_value: f64
// ) 