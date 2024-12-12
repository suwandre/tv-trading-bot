use std::sync::Arc;

use axum::{Extension, Json};
use hyper::StatusCode;
use mongodb::{bson::{doc, oid::ObjectId, Document}, results::{DeleteResult, InsertOneResult, UpdateResult}, Cursor};
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
/// 
/// Only one paper trade can exist for a given pair at a time, regardless of direction. If a new alert is received and is the opposite direction of the current trade,
/// the current trade will be closed (a new one will NOT be opened). The next incoming alert will then determine the new trade's direction.
pub async fn execute_paper_trade(
    Extension(mongo_state): Extension<Arc<MongoDBState>>, 
    payload: Json<Value>
) -> (StatusCode, Json<ApiResponse<()>>) {
    println!("Received payload: {:?}", payload);

    match serde_json::from_value::<TradingViewAlert>(payload.0) {
        Ok(alert) => {
            let expected_secret = std::env::var("TRADINGVIEW_SECRET").expect("(execute_paper_trade) TRADINGVIEW_SECRET must be set");

            if alert.secret != expected_secret {
                eprintln!("(execute_paper_trade) Invalid secret provided.");

                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse {
                        status: "401 Unauthorized",
                        message: "(execute_paper_trade) Invalid secret provided.".to_string(),
                        data: None
                    })
                )
            }

            // // a check needs to be made to ensure that an active trade with the same pair doesn't already exist
            // // if it does exist:
            // // 1. if the direction is the same, do nothing (i.e. ignore the alert).
            // // 2. if the direction is the opposite, close the current trade.
            // if let Ok(Some(existing_trade)) = mongo

            (
                StatusCode::OK,
                Json(ApiResponse {
                    status: "200 OK",
                    message: "(execute_paper_trade) Trade executed successfully.".to_string(),
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
                    message: format!("(execute_paper_trade) Failed to deserialize payload: {}", err),
                    data: None
                })
            )
        }
    }
}