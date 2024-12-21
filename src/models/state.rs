use std::sync::Arc;

use crate::api::ActiveTradesMap;

use super::MongoDBState;

/// A global application state struct which can be shared across handlers, WebSockets, etc.
pub struct AppState {
    /// The MongoDB data-access object.
    pub mongo_state: Arc<MongoDBState>,

    /// All active trades in memory (for real-time checks).
    pub active_trades: ActiveTradesMap,
}