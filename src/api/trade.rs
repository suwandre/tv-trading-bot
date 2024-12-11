use axum::Json;
use hyper::StatusCode;
use serde_json::Value;

use crate::models::{tradingview::TradingViewAlert, ApiResponse, TradeSignal};

/// Executes a paper trade based on the alert received from TradingView.
/// 
/// A paper trade will NOT use real money and will only be used for the purpose of recording/testing trades.
pub async fn execute_paper_trade(payload: Json<Value>) -> (StatusCode, Json<ApiResponse<()>>) {
    println!("Received payload: {:?}", payload);

    match serde_json::from_value::<TradingViewAlert>(payload.0) {
        Ok(alert) => {
            let expected_secret = std::env::var("TRADINGVIEW_SECRET").expect("TRADINGVIEW_SECRET must be set");

            if alert.secret != expected_secret {
                eprintln!("Invalid secret provided.");

                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse {
                        status: "401 Unauthorized",
                        message: "Invalid secret provided.".to_string(),
                        data: None
                    })
                )
            }

            if alert.signal == TradeSignal::Buy {
                println!("Executing buy order for alert {} for pair {} at {}", alert.name, alert.pair, alert.price);
            } else if alert.signal == TradeSignal::Sell {
                println!("Executing sell order for alert {} for pair {} at {}", alert.name, alert.pair, alert.price);
            }

            (
                StatusCode::OK,
                Json(ApiResponse {
                    status: "200 OK",
                    message: "Trade executed successfully.".to_string(),
                    data: None
                })
            )
        }
        Err(err) => {
            eprintln!("(execute_trade) Failed to deserialize payload: {}", err);

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponse {
                    status: "422 Unprocessable Entity",
                    message: format!("Failed to deserialize payload: {}", err),
                    data: None
                })
            )
        }
    }
}