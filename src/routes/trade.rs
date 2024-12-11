use axum::{routing::post, Router};

use crate::api::trade::execute_paper_trade;

pub fn trade_routes() -> Router {
    Router::new().route("/execute_paper_trade", post(execute_paper_trade))
}