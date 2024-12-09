use axum::{
    routing::get,
    Router
};

/// Checks to see if the server is running
async fn run_axum() -> &'static str {
    "Axum is Running"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(run_axum));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
