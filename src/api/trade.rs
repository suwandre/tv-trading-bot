use axum::{response::IntoResponse, Json};

use crate::models::{tradingview::TradingViewAlert, TradeSignal};

/// Executes a trade based on the alert received from TradingView.
pub async fn execute_trade(Json(alert): Json<TradingViewAlert>) -> impl IntoResponse {
    println!("Received alert: {:?}", alert);

    let expected_secret = std::env::var("TRADINGVIEW_SECRET").expect("TRADINGVIEW_SECRET must be set");

    if alert.secret != expected_secret {
        return axum::response::Response::builder()
            .status(axum::http::StatusCode::UNAUTHORIZED)
            .body(axum::body::Body::from("Invalid secret provided"))
            .unwrap();
    }

    if alert.signal == TradeSignal::Buy {
        println!("Executing buy order for {} at {}", alert.pair, alert.price);
    } else if alert.signal == TradeSignal::Sell {
        println!("Executing sell order for {} at {}", alert.pair, alert.price);
    }

    axum::response::Response::builder()
        .status(axum::http::StatusCode::OK)
        .body(axum::body::Body::from("Trade executed successfully"))
        .unwrap()
}