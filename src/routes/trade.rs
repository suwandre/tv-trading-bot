use std::sync::Arc;

use axum::{routing::post, Extension, Router};

use crate::{api::trade::execute_paper_trade, models::MongoDBState};

pub fn trade_routes(mongo_state: Arc<MongoDBState>) -> Router {
    Router::new()
        .route("/execute_paper_trade", post(execute_paper_trade))
        .layer(Extension(mongo_state))
}