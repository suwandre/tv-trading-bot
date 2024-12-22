use std::{collections::HashMap, sync::{Arc, Mutex}};

use axum::{Extension, Json};
use chrono::Utc;
use hyper::StatusCode;
use mongodb::{bson::{doc, oid::ObjectId, to_bson, Document}, results::{DeleteResult, InsertOneResult, UpdateResult}, Cursor};
use serde_json::Value;

use crate::{api::{calc_final_execution_fees, calc_final_funding_fees, calc_liquidation_price, calc_pnl, calc_roe}, constants::{DEFAULT_LEVERAGE, DEFAULT_NOTIONAL_VALUE, MAX_PER_PAGE}, models::{tradingview::TradingViewAlert, ActiveTrade, ApiResponse, AppState, ClosedTrade, MongoDBState, TradeKind, TradeLeverage, TradeSignal}};

/// A thread-safe map of active trades in memory.
pub type ActiveTradesMap = Arc<Mutex<HashMap<ObjectId, ActiveTrade>>>;

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

    /// Fetches an active trade from the database based on the provided alert name, pair and kind (APK).
    pub async fn fetch_active_trade_by_apk(
        &self, 
        alert_name: &String,
        pair: &String, 
        kind: &TradeKind,
    ) -> Result<Option<ActiveTrade>, mongodb::error::Error> {
        // convert TradeKind to Bson
        let kind_bson = to_bson(&kind).map_err(|e| mongodb::error::Error::from(e))?;

        self.active_trade_collection.find_one(doc! { "alertName": alert_name, "pair": pair, "kind": kind_bson }).await
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

            // a check needs to be made to ensure that an active trade with the same pair, kind AND alert name doesn't already exist
            // if it does exist:
            // 1. if the direction is the same, do nothing (i.e. ignore the alert).
            // 2. if the direction is the opposite, close the current trade and open a new one in this direction.
            // if it doesn't exist, proceed to open a new trade.
            if let Ok(Some(existing_trade)) = mongo_state.fetch_active_trade_by_apk(&alert.name, &alert.pair, &TradeKind::Paper).await {
                println!("(execute_paper_trade) Existing trade found: {:?}", existing_trade);

                if existing_trade.direction == alert.signal.into() {
                    println!("(execute_paper_trade) Alert signal matches existing trade direction. Ignoring alert.");

                    return (
                        StatusCode::OK,
                        Json(ApiResponse {
                            status: "200 OK",
                            message: "(execute_paper_trade) Alert signal matches existing trade direction. Ignoring alert.".to_string(),
                            data: None
                        })
                    )
                } else {
                    println!("(execute_paper_trade) Alert signal is opposite of existing trade direction. Closing existing trade and opening a new one.");
                    
                    let execution_fees = calc_final_execution_fees(
                        existing_trade.quantity,
                        existing_trade.entry_price
                    );

                    let funding_fees = calc_final_funding_fees(
                        existing_trade.open_timestamp,
                        Utc::now(),
                        ((existing_trade.quantity * existing_trade.entry_price) + (existing_trade.quantity * alert.price)) / 2.0
                    );

                    let pnl = calc_pnl(
                        existing_trade.entry_price,
                        alert.price,
                        existing_trade.quantity,
                        execution_fees,
                        funding_fees,
                        &existing_trade.direction,
                    );
                    
                    let roe = calc_roe(
                        pnl,
                        existing_trade.entry_price,
                        existing_trade.quantity,
                        existing_trade.leverage.into()
                    );

                    // close the existing trade and add it to the closed trades collection
                    let closed_trade = ClosedTrade {
                        id: existing_trade.id,
                        alert_name: alert.name.clone(),
                        pair: existing_trade.pair,
                        direction: existing_trade.direction,
                        kind: existing_trade.kind,
                        quantity: existing_trade.quantity,
                        entry_price: existing_trade.entry_price,
                        exit_price: alert.price,
                        leverage: existing_trade.leverage,
                        liquidation_price: existing_trade.liquidation_price,
                        open_timestamp: existing_trade.open_timestamp,
                        close_timestamp: Utc::now(),
                        pnl,
                        roe,
                        // get the opening fee and add the closing fee
                        execution_fees,
                        // funding fee is simplified and estimated based on entry and exit prices
                        funding_fees,
                    };

                    // add the closed trade to the database. since this is a paper trade, no need to 
                    // call any API to close the trade on the exchange.
                    match mongo_state.add_closed_trade(closed_trade).await {
                        Ok(_) => {
                            // delete the existing trade from the active trades collection
                            match mongo_state.delete_active_trade(existing_trade.id).await {
                                Ok(_) => {
                                    println!("(execute_paper_trade) Closed existing trade and added to closed trades collection. Now creating a new trade.");

                                    // create a new trade based on the alert on the opposite direction
                                    let new_active_trade = ActiveTrade {
                                        id: ObjectId::new(),
                                        alert_name: alert.name,
                                        pair: alert.pair,
                                        direction: alert.signal.into(),
                                        kind: TradeKind::Paper,
                                        open_timestamp: Utc::now(),
                                        quantity: (DEFAULT_NOTIONAL_VALUE / alert.price * 100.0).round() / 100.0, // rounded to 2 dp
                                        entry_price: alert.price,
                                        leverage: DEFAULT_LEVERAGE,
                                        liquidation_price: calc_liquidation_price(alert.price, DEFAULT_LEVERAGE.into(), &alert.signal.into()),
                                        take_profit: alert.take_profit,
                                        stop_loss: alert.stop_loss,
                                    };

                                    // add the new trade to the active trades collection
                                    match mongo_state.add_active_trade(new_active_trade).await {
                                        Ok(_) => {
                                            println!("(execute_paper_trade) Opened new trade successfully.");

                                            return (
                                                StatusCode::OK,
                                                Json(ApiResponse {
                                                    status: "200 OK",
                                                    message: "(execute_paper_trade) Closed existing trade and added to closed trades collection. Also opened new trade successfully.".to_string(),
                                                    data: None
                                                })
                                            )
                                        }
                                        Err(err) => {
                                            eprintln!("(execute_paper_trade) Failed to open new trade: {}", err);

                                            return (
                                                StatusCode::INTERNAL_SERVER_ERROR,
                                                Json(ApiResponse {
                                                    status: "500 Internal Server Error",
                                                    message: format!("(execute_paper_trade) Failed to open new trade: {}", err),
                                                    data: None
                                                })
                                            )
                                        }
                                    }
                                }
                                Err(err) => {
                                    eprintln!("(execute_paper_trade) Failed to delete existing trade: {}", err);

                                    return (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(ApiResponse {
                                            status: "500 Internal Server Error",
                                            message: format!("(execute_paper_trade) Failed to delete existing trade: {}", err),
                                            data: None
                                        })
                                    )
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("(execute_paper_trade) Failed to add closed trade: {}", err);

                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ApiResponse {
                                    status: "500 Internal Server Error",
                                    message: format!("(execute_paper_trade) Failed to add closed trade: {}", err),
                                    data: None
                                })
                            )
                        }
                    }
                }
            // if no existing trade is found, proceed to open a new paper trade
            } else {
                println!("(execute_paper_trade) No existing trade found. Proceeding to open new trade.");

                let active_trade = ActiveTrade {
                    id: ObjectId::new(),
                    alert_name: alert.name,
                    pair: alert.pair,
                    direction: alert.signal.into(),
                    kind: TradeKind::Paper,
                    open_timestamp: Utc::now(),
                    quantity: (DEFAULT_NOTIONAL_VALUE / alert.price * 100.0).round() / 100.0, // rounded to 2 dp
                    entry_price: alert.price,
                    leverage: DEFAULT_LEVERAGE,
                    liquidation_price: calc_liquidation_price(alert.price, DEFAULT_LEVERAGE.into(), &alert.signal.into()),
                    take_profit: alert.take_profit,
                    stop_loss: alert.stop_loss,
                };

                match mongo_state.add_active_trade(active_trade).await {
                    Ok(_) => {
                        println!("(execute_paper_trade) Opened new trade successfully.");

                        return (
                            StatusCode::OK,
                            Json(ApiResponse {
                                status: "200 OK",
                                message: "(execute_paper_trade) Opened new trade successfully.".to_string(),
                                data: None
                            })
                        )
                    }
                    Err(err) => {
                        eprintln!("(execute_paper_trade) Failed to open new trade: {}", err);

                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse {
                                status: "500 Internal Server Error",
                                message: format!("(execute_paper_trade) Failed to open new trade: {}", err),
                                data: None
                            })
                        )
                    }
                }
            }
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


/// Closes an active paper trade if either:
/// 1) The take profit price is hit.
/// 2) The stop loss price is hit.
/// 3) The liquidation price is hit.
/// - Removes from in-memory
/// - Moves to closed trades collection in DB
pub async fn close_paper_trade(
    app_state: &AppState, 
    trade_id: &ObjectId,
    exit_price: f64
) {
    // remove from in-memory so we don't close it twice
    {
        let mut map = app_state.active_trades.lock().unwrap();
        map.remove(trade_id);
    }

    // 2. Insert into "closed trades" + remove from "active trades" in DB
    //    Reuse your existing logic (calc fees, PnL, etc.)
    //    or replicate your existing "closing" code.
    //    For example:
    //        let closed_trade = ClosedTrade { ... };
    //        app_state.mongo_state.add_closed_trade(closed_trade).await?;
    //        app_state.mongo_state.delete_active_trade(trade.id).await?;

    println!("Trade {} closed at price {}", trade_id, exit_price);
}