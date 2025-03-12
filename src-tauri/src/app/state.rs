use std::sync::{Mutex,Arc};
use crate::store::setting::Configuration;
use crate::store::db::Database;
use super::updater::PendingUpdate;
#[derive(Clone)]
pub struct AppState {
    pub config :Arc<Mutex<Configuration>>,
    pub db: Arc<Database>,
    pub pending_update: Arc<Mutex<PendingUpdate>>,
}