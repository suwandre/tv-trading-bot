use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use mongodb::{bson::oid::ObjectId, options::ClientOptions, Client};

use crate::models::{ActiveTrade, MongoDBState, TradeDirection, TradeKind, TradeLeverage};

#[tokio::test]
pub async fn add_active_trade() {
    dotenv().ok();

    let mongodb_uri = std::env::var("MONGODB_URI").expect("(add_active_trade) MONGODB_URI not set");
    let client_options = ClientOptions::parse(mongodb_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("main");

    let state = MongoDBState {
        active_trade_collection: db.collection("ActiveTrades"),
        closed_trade_collection: db.collection("ClosedTrades"),
    };

    let sample_trade = ActiveTrade {
        id: ObjectId::new(),
        pair: "SOLUSDT".to_string(),
        direction: TradeDirection::Long,
        kind: TradeKind::Live,
        open_timestamp: Utc::now(),
        quantity: 100.0,
        entry_price: 231.4,
        leverage: TradeLeverage::One,
        take_profit: Some(240.0),
        stop_loss: Some(225.0),
        liquidation_price: 10.0,
    };

    match state.add_active_trade(sample_trade).await {
        Ok(result) => println!("(add_active_trade) Inserted document ID: {:?}", result.inserted_id),
        Err(e) => eprintln!("(add_active_trade) Error: {:?}", e)
    }
}