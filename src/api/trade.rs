use axum::Json;
use hyper::StatusCode;
use mongodb::{bson::{self, doc, from_document, oid::ObjectId, Document}, options::FindOptions, results::{DeleteResult, InsertOneResult, UpdateResult}, Cursor};
use serde_json::Value;

use crate::{constants::MAX_PER_PAGE, models::{tradingview::TradingViewAlert, ActiveTrade, ApiResponse, ClosedTrade, MongoDBState, TradeSignal}};

/// CRUD operations for active and closed trades in the database.
impl MongoDBState {
    /// Adds an active trade instance into the database. Called when a trade is executed.
    pub async fn add_active_trade(&self, trade: ActiveTrade) -> Result<InsertOneResult, mongodb::error::Error> {
        self.active_trade_collection.insert_one(trade).await
    }

    /// Fetches all active trades with pagination and optional filtering
    pub async fn fetch_active_trades(
        &self, 
        // optional filter
        filter: Option<Document>,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<ActiveTrade>, mongodb::error::Error> {
        let per_page = per_page.min(MAX_PER_PAGE as u32); // ensure per_page is within the limit `MAX_PER_PAGE`
        let skip = (page - 1) * per_page;

        let mut cursor: Cursor<ActiveTrade> = self
            .active_trade_collection
            .find(filter.unwrap_or_default())
            .skip(skip as u64)
            .limit(per_page as i64)
            .await?;
        
        let mut results = Vec::new();

        while cursor.advance().await? {
            let trade = cursor.deserialize_current()?;

            results.push(trade);
        }

        Ok(results)
    }

    /// Fetches an active trade from the database based on the provided ID.
    pub async fn fetch_active_trade(&self, id: ObjectId) -> Result<Option<ActiveTrade>, mongodb::error::Error> {
        self.active_trade_collection.find_one(doc! { "_id": id }).await
    }

    /// Updates an active trade in the database based on the provided ID.
    pub async fn update_active_trade(&self, id: ObjectId, update: Document) -> Result<UpdateResult, mongodb::error::Error> {
        self.active_trade_collection.update_one(doc! { "_id": id }, update).await
    }

    /// Deletes an active trade from the database based on the provided ID.
    pub async fn delete_active_trade(&self, id: ObjectId) -> Result<DeleteResult, mongodb::error::Error> {
        self.active_trade_collection.delete_one(doc! { "_id": id }).await
    }

    /// Adds a closed trade instance into the database. Called when a trade is closed.
    pub async fn add_closed_trade(&self, trade: ClosedTrade) -> Result<InsertOneResult, mongodb::error::Error> {
        self.closed_trade_collection.insert_one(trade).await
    }

    /// Fetches all closed trades with pagination and optional filtering
    pub async fn fetch_closed_trades(
        &self, 
        // optional filter
        filter: Option<Document>,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<ClosedTrade>, mongodb::error::Error> {
        let per_page = per_page.min(MAX_PER_PAGE as u32); // ensure per_page is within the limit `MAX_PER_PAGE`
        let skip = (page - 1) * per_page;

        let mut cursor: Cursor<ClosedTrade> = self
            .closed_trade_collection
            .find(filter.unwrap_or_default())
            .skip(skip as u64)
            .limit(per_page as i64)
            .await?;
        
        let mut results = Vec::new();

        while cursor.advance().await? {
            let trade = cursor.deserialize_current()?;

            results.push(trade);
        }

        Ok(results)
    }

    /// Fetches a closed trade from the database based on the provided ID.
    pub async fn fetch_closed_trade(&self, id: ObjectId) -> Result<Option<ClosedTrade>, mongodb::error::Error> {
        self.closed_trade_collection.find_one(doc! { "_id": id }).await
    }

    /// Updates a closed trade in the database based on the provided ID.
    pub async fn update_closed_trade(&self, id: ObjectId, update: Document) -> Result<UpdateResult, mongodb::error::Error> {
        self.closed_trade_collection.update_one(doc! { "_id": id }, update).await
    }

    /// Deletes a closed trade from the database based on the provided ID.
    pub async fn delete_closed_trade(&self, id: ObjectId) -> Result<DeleteResult, mongodb::error::Error> {
        self.closed_trade_collection.delete_one(doc! { "_id": id }).await
    }
}

/// Executes a paper trade based on the alert received from TradingView.
/// 
/// A paper trade will NOT use real money and will only be used for the purpose of recording/testing trades.
pub async fn execute_paper_trade(payload: Json<Value>) -> (StatusCode, Json<ApiResponse<()>>) {
    println!("Received payload: {:?}", payload);

    match serde_json::from_value::<TradingViewAlert>(payload.0) {
        Ok(alert) => {
            let expected_secret = std::env::var("TRADINGVIEW_SECRET").expect("TRADINGVIEW_SECRET must be set");

            if alert.secret != expected_secret {
                eprintln!("Invalid secret provided.");

                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse {
                        status: "401 Unauthorized",
                        message: "Invalid secret provided.".to_string(),
                        data: None
                    })
                )
            }

            if alert.signal == TradeSignal::Buy {
                println!("Executing buy order for alert {} for pair {} at {}", alert.name, alert.pair, alert.price);
            } else if alert.signal == TradeSignal::Sell {
                println!("Executing sell order for alert {} for pair {} at {}", alert.name, alert.pair, alert.price);
            }

            (
                StatusCode::OK,
                Json(ApiResponse {
                    status: "200 OK",
                    message: "Trade executed successfully.".to_string(),
                    data: None
                })
            )
        }
        Err(err) => {
            eprintln!("(execute_trade) Failed to deserialize payload: {}", err);

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponse {
                    status: "422 Unprocessable Entity",
                    message: format!("Failed to deserialize payload: {}", err),
                    data: None
                })
            )
        }
    }
}