use std::{collections::HashMap, sync::{Arc, Mutex}};

use mongodb::bson::oid::ObjectId;

use crate::models::{ActiveTrade, AppState, MongoDBState};

impl AppState {
    /// Initialize a new `AppState`.
    pub fn new(mongo_state: Arc<MongoDBState>) -> Self {
        Self {
            mongo_state,
            active_trades: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}