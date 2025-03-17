use crate::app::{state::AppState, APP};
use crate::model::{FavoriteMessage, MessageItem};
use tauri::{Emitter, Manager, State};

use crate::store::module::FavoriteManager;
#[tauri::command]
pub async fn add_new_favorite(
    state: State<'_, AppState>,
    message: MessageItem,
) -> Result<i64, String> {
    let api = state
        .config
        .try_lock()
        .expect("get config of state error")
        .api
        .clone();
    match state
        .db
        .favorite_message(&message, api.model)
        .map_err(|e| e.to_string())
    {
        Ok(id) => {
            let handle = APP.get().take().unwrap().app_handle();
            handle.emit("refresh_favorite", ()).unwrap();
            Ok(id)
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn remove_favorite(state: State<'_, AppState>, message_id: i64) -> Result<(), String> {
    match state
        .db
        .unfavorite_message(message_id)
        .map_err(|e| e.to_string())
    {
        Ok(_) => {
            let handle = APP.get().take().unwrap().app_handle();
            handle.emit("refresh_favorite", ()).unwrap();
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn get_favorites(state: State<'_, AppState>) -> Result<Vec<FavoriteMessage>, String> {
    state.db.get_favorited_messages().map_err(|e| e.to_string())
}
