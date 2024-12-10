use axum::Json;
use hyper::StatusCode;

use crate::models::{tradingview::TradingViewAlert, ApiResponse, TradeSignal};

/// Executes a trade based on the alert received from TradingView.
pub async fn execute_trade(Json(alert): Json<TradingViewAlert>) -> (StatusCode, Json<ApiResponse<()>>) {
    println!("Received alert: {:?}", alert);

    let expected_secret = std::env::var("TRADINGVIEW_SECRET").expect("TRADINGVIEW_SECRET must be set");

    if alert.secret != expected_secret {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                // include both `401` and `Unauthorized` in the response
                status: "401 Unauthorized",
                message: "Invalid secret provided.",
                data: None
            })
        )
    }

    if alert.signal == TradeSignal::Buy {
        println!("Executing buy order for {} at {}", alert.pair, alert.price);
    } else if alert.signal == TradeSignal::Sell {
        println!("Executing sell order for {} at {}", alert.pair, alert.price);
    }

    (
        StatusCode::OK,
        Json(ApiResponse {
            // include both `200` and `OK` in the response
            status: "200 OK",
            message: "Trade executed successfully.",
            data: None
        })
    )
}