use std::sync::Arc;
use mongodb::{bson::doc, options::ClientOptions, Client};

use crate::models::{ActiveTrade, ClosedTrade, MongoDBState};

impl MongoDBState {
    /// Initializes a new MongoDBState instance with the provided client and required collections.
    pub fn new(client: Arc<Client>) -> Self {
        let active_trade_collection = client.database("main").collection::<ActiveTrade>("ActiveTrades");
        let closed_trade_collection = client.database("main").collection::<ClosedTrade>("ClosedTrades");

        Self {
            active_trade_collection,
            closed_trade_collection
        }
    }
}

/// Initializes a MongoDB client, returning `Arc<Client>` for sharing across threads.
pub async fn init_mongo(uri: &str) -> mongodb::error::Result<Arc<Client>> {
    let client_options = ClientOptions::parse(uri).await?;

    // creates a new client (wrapped in Arc for thread-safe sharing)
    let client = Client::with_options(client_options)?;

    // database ping to ensure the connection is live
    client.database("admin").run_command(doc! { "ping": 1 }).await?;

    println!("MongoDB connected successfully!");
    Ok(Arc::new(client))
}