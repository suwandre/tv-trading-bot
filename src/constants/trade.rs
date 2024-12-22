use crate::models::TradeLeverage;

/// Accepted symbols to trade on and receive WebSocket subscriptions for.
/// 
/// Prevents unwanted symbols from being traded on or subscribed to.
pub const ACCEPTED_SYMBOLS: &[&str] = &[
    "BTCUSDT",
    "ETHUSDT",
    "BNBUSDT",
    "SOLUSDT",
];

/// Fee for opening and closing a trade (in percentage format). Used in paper trades only to simulate real trading fees.
pub const EXECUTION_FEE_PERCENTAGE: f64 = 0.05;

/// Funding fee for holding a trade over 8 hours (in percentage format). Used in paper trades only to simulate real funding fees.
/// 
/// Negative funding fees are paid by the shorters to longers. Positive funding fees are paid by longers to shorters.
/// 
/// If a trade is held for more than 8 hours, the funding fee will start accumulating based on this percentage.
pub const FUNDING_FEE_8H_PERCENTAGE: f64 = 0.01;

/// The hours (in UTC) at which the funding fee will start accumulating. Used in paper trades only to simulate real funding fees.
/// 
/// If a trade is opened, say, 07:59 UTC, the funding fee will start accumulating at 08:00 UTC.
pub const FUNDING_FEE_HOURS: [u8; 3] = [0, 8, 16];

/// The margin required (in percentage) of the notional value to keep the trade open and prevent liquidation. 
/// Used in paper trades only to simulate real margin requirements.
pub const MAINTENANCE_MARGIN: f64 = 1.0;

/// The default total value of a trade upon entry (in USDT). Used in paper trades only to simulate real trades.
/// 
/// Therefore, the quantity of the base currency will be calculated based on this value and the entry price.
pub const DEFAULT_NOTIONAL_VALUE: f64 = 1000.0;

/// The default leverage used for a trade. Used in paper trades only to simulate real trades.
pub const DEFAULT_LEVERAGE: TradeLeverage = TradeLeverage::Three;

/// The default take profit percentage to set for a trade. Used in paper trades only to simulate real trades.
///
/// This is only used if the alert does not provide a take profit price.
pub const DEFAULT_TAKE_PROFIT_PERCENTAGE: f64 = 5.0;

/// The default stop loss percentage to set for a trade. Used in paper trades only to simulate real trades.
/// 
/// This is only used if the alert does not provide a stop loss price.
pub const DEFAULT_STOP_LOSS_PERCENTAGE: f64 = 2.0;