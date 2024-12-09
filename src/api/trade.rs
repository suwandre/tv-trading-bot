use axum::{response::IntoResponse, Json};

use crate::models::{tradingview::TradingViewAlert, TradeSignal};

/// Executes a trade based on the alert received from TradingView.
pub async fn execute_trade(Json(alert): Json<TradingViewAlert>) -> impl IntoResponse {
    println!("Received alert: {:?}", alert);

    if alert.signal == TradeSignal::Buy {
        println!("Executing buy order for {} at {}", alert.symbol, alert.price);
    } else if alert.signal == TradeSignal::Sell {
        println!("Executing sell order for {} at {}", alert.symbol, alert.price);
    }

    "Trade executed"
}