use mongodb::Collection;

use super::{ActiveTrade, ClosedTrade};

/// A struct that manages MongoDB collections and provide shared access across the app.
pub struct MongoDBState {
    pub active_trade_collection: Collection<ActiveTrade>,
    pub closed_trade_collection: Collection<ClosedTrade>,
}