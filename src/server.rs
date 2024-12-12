mod models;
mod api;
mod routes;
mod configs;

use std::{net::SocketAddr, sync::Arc};
use axum::{
    routing::get, Extension, Router
};
use dotenvy::dotenv;
use configs::init_mongo;
use models::MongoDBState;
use routes::trade_routes;

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
    let mongo_state = MongoDBState::new(mongo_client.clone());

    let app = Router::new()
        .route("/", get(run_axum))
        // add trade routes
        .nest("/trade", trade_routes())
        // wrap `mongo_state` in an Arc again because the struct itself isn't wrapped in an Arc even if the cloned client is
        .layer(Extension(Arc::new(mongo_state)));

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