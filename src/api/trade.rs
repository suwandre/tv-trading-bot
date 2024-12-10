use axum::Json;
use hyper::StatusCode;

use crate::models::{tradingview::TradingViewAlert, ApiResponse, TradeSignal};

/// Executes a trade based on the alert received from TradingView.
pub async fn execute_trade(payload: axum::extract::Json<serde_json::Value>) -> (StatusCode, Json<ApiResponse<()>>) {
    println!("Raw payload: {:?}", payload);

    // Return OK for testing
    (
        StatusCode::OK,
        Json(ApiResponse {
            status: "200 OK",
            message: "Payload received successfully.",
            data: None,
        }),
    )
}
// pub async fn execute_trade(Json(alert): Json<TradingViewAlert>) -> (StatusCode, Json<ApiResponse<()>>) {
//     println!("Received alert: {:?}", alert);

//     let expected_secret = std::env::var("TRADINGVIEW_SECRET").expect("TRADINGVIEW_SECRET must be set");

//     if alert.secret != expected_secret {
//         return (
//             StatusCode::UNAUTHORIZED,
//             Json(ApiResponse {
//                 status: "401 Unauthorized",
//                 message: "Invalid secret provided.",
//                 data: None
//             })
//         )
//     }

//     if alert.signal == TradeSignal::Buy {
//         println!("Executing buy order for alert {} for pair {} at {}", alert.name, alert.pair, alert.price);
//     } else if alert.signal == TradeSignal::Sell {
//         println!("Executing sell order for alert {} for pair {} at {}", alert.name, alert.pair, alert.price);
//     }

//     (
//         StatusCode::OK,
//         Json(ApiResponse {
//             status: "200 OK",
//             message: "Trade executed successfully.",
//             data: None
//         })
//     )
// }