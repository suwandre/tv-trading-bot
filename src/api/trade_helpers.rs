use chrono::{DateTime, Duration, Timelike, Utc};

use crate::{constants::{EXECUTION_FEE_PERCENTAGE, FUNDING_FEE_8H_PERCENTAGE, FUNDING_FEE_HOURS, MAINTENANCE_MARGIN}, models::TradeDirection};

/// Calculate the Profit and Loss (PnL) for a trade.
pub fn calc_pnl(
    entry_price: f64,
    exit_price: f64,
    quantity: f64,
    execution_fees: f64,
    funding_fees: f64,
    direction: TradeDirection
) -> f64 {
    let raw_pnl = if direction == TradeDirection::Long {
        (exit_price - entry_price) * quantity
    } else {
        (entry_price - exit_price) * quantity
    };

    raw_pnl - execution_fees - funding_fees
}

/// Calculates the Return on Equity (ROE) for a trade.
pub fn calc_roe(
    pnl: f64,
    entry_price: f64,
    quantity: f64,
    leverage: f64
) -> f64 {
    // calculate margin (equity used)
    let notional_value = entry_price * quantity;
    let margin = notional_value / leverage;

    // return ROE as percentage
    (pnl / margin) * 100.0
}

/// Calculate the liquidation price of a trade based on the entry price, leverage, and direction. Used for both long and short trades.
/// 
/// Only used primarily in paper trading to simulate real liquidation prices.
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

/// Calculates the final funding fees for a trade, taking into account the funding fee percentage, the duration and the average notional value of the trade.
/// 
/// Used only in paper trading to simulate real funding fees.
/// 
/// Normally, funding fees are calculated with the notional value at the time of funding. However, for paper trading, this function will only be called
/// once when the trade is closed. Therefore, the average notional value of the trade between opening and closing will be used, purely for estimation.
pub fn calc_final_funding_fees(
    open_timestamp: DateTime<Utc>, 
    close_timestamp: DateTime<Utc>,
    // the average margin/notional value of the position between opening and closing the trade.
    // calculated by (initial margin + final margin) / 2
    average_notional_value: f64
) -> f64 {
    // edge case: no funding fees if the trade duration is zero or somehow negative
    if open_timestamp >= close_timestamp {
        return 0.0;
    }

    // init final funding fees
    let mut final_funding_fees = 0.0;

    // start from the first funding interval after `open_timestamp`
    let mut current_funding_time = get_next_funding_time(open_timestamp);

    while current_funding_time <= close_timestamp {
        // add the funding fee for this interval
        final_funding_fees += average_notional_value * (FUNDING_FEE_8H_PERCENTAGE / 100.0);

        // move on to the next funding interval
        current_funding_time += Duration::hours(8);
    }

    final_funding_fees
}

/// Get the next funding time after a given timestamp.
pub fn get_next_funding_time(timestamp: DateTime<Utc>) -> DateTime<Utc> {
    let date = timestamp.date_naive();
    let hour = timestamp.hour();

    // find the next funding hour today
    for &funding_hour in &FUNDING_FEE_HOURS {
        if hour < funding_hour.into() {
            if let Some(next_time) = date.and_hms_opt(funding_hour.into(), 0, 0) {
                return DateTime::from_naive_utc_and_offset(next_time, Utc);
            }
        }
    }

    // if no funding hour is found today, return the first funding hour of the next day
    if let Some(next_day_time) = (date + Duration::days(1)).and_hms_opt(FUNDING_FEE_HOURS[0].into(), 0, 0) {
        return DateTime::from_naive_utc_and_offset(next_day_time, Utc);
    }

    // this point should never be reached if funding hours are correctly configured
    panic!("(get_next_funding_time) No valid funding times configured");
}