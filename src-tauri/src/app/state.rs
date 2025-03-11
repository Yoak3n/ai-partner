use std::sync::{Mutex,Arc};
use crate::store::setting::Configuration;
use crate::store::db::Database;
#[derive(Clone)]
pub struct AppState {
    pub config :Arc<Mutex<Configuration>>,
    pub db: Arc<Database>,
}