use std::sync::Arc;
use mongodb::{bson::doc, options::ClientOptions, Client};

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