mod models;
mod api;
mod routes;
mod configs;
mod constants;
mod tests;

use std::{net::SocketAddr, sync::Arc};
use api::{start_price_listener, websocket};
use axum::{
    routing::get, Extension, Router
};
use dotenvy::dotenv;
use configs::init_mongo;
use models::{AppState, MongoDBState};
use routes::trade_routes;
use tokio::sync::mpsc;

/// Checks to see if the server is running
async fn run_axum() -> &'static str {
    "Axum is Running"
}   

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_uri = std::env::var("MONGODB_URI").expect("MONGO_URI must be set");
    let mongo_client = init_mongo(&mongo_uri).await.expect("Failed to initialize MongoDB client");
    // initialize a mongo state (with the required collections) with the initialized client
    // wrap in an Arc again because the struct itself isn't wrapped in an Arc even if the cloned client is
    let mongo_state = Arc::new(MongoDBState::new(mongo_client.clone()));

    // initialize and build an app state
    let app_state = Arc::new(AppState::new(mongo_state.clone()));

    // preload any existing trades from the database into in-memory
    if let Ok(existing_trades) = mongo_state.fetch_active_trades(None, 1, 1000).await {
        let mut map = app_state.active_trades.lock().unwrap();
        for t in existing_trades {
            map.insert(t.id.clone(), t);
        }
    }

    let app_state_for_ws = app_state.clone();
    tokio::spawn(async move {
        start_price_listener(app_state_for_ws).await;
    });

    let app = Router::new()
        .route("/", get(run_axum))
        // add trade routes
        .nest("/trade", trade_routes(mongo_state.clone()))
        .layer(Extension(app_state))
        .layer(Extension(mongo_state));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Invalid PORT");

    // bind to 0.0.0.0:<PORT or 3000>
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Server running on: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}