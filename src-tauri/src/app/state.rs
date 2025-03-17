use super::updater::PendingUpdate;
use crate::store::db::Database;
use crate::store::setting::Configuration;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Mutex<Configuration>>,
    pub db: Arc<Database>,
    pub pending_update: Arc<Mutex<PendingUpdate>>,
}
