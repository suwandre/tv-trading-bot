use std::sync::Arc;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};

use crate::models::{ActiveTrade, ClosedTrade, MongoDBState};

impl MongoDBState<'_> {
    /// Creates a new `MongoDBState` instance, initializing the necessary collections.
    /// 
    /// This method takes an `Arc<Client>` to ensure the MongoDB client can be shared
    /// safely across multiple threads. It initializes the database and its collections,
    /// allowing the app to perform CRUD operations on them.
    pub fn new(client: Arc<Client>) -> Self {
        let db = client.database("main");
        let active_trade_collection = db.collection::<ActiveTrade>("ActiveTrades");
        let closed_trade_collection = db.collection::<ClosedTrade>("ClosedTrades");

        Self {
            active_trade_collection,
            closed_trade_collection,
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