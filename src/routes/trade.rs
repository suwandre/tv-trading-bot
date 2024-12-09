use axum::{routing::post, Router};

use crate::api::trade::execute_trade;

pub fn trade_routes() -> Router {
    Router::new().route("/execute_trade", post(execute_trade))
}