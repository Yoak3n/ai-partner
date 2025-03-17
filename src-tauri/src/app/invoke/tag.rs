use tauri::{Emitter, Manager, State};

use crate::app::{state::AppState, APP};
use crate::model::{table::Tag, FavoriteMessage};
use crate::store::module::TagManager;
#[tauri::command]
pub async fn add_tag(
    state: State<'_, AppState>,
    name: String,
    color: String,
) -> Result<(), String> {
    match state
        .db
        .create_tag(&name, &color)
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
pub async fn get_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, String> {
    state.db.get_tags().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_tag_to_message(
    state: State<'_, AppState>,
    message_id: usize,
    tag_id: i64,
) -> Result<(), String> {
    state
        .db
        .add_tag_to_message(message_id, tag_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_message_tags(
    state: State<'_, AppState>,
    message_id: usize,
) -> Result<Vec<Tag>, String> {
    state
        .db
        .get_message_tags(message_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_messages_by_tag(
    state: State<'_, AppState>,
    tag_id: i64,
) -> Result<Vec<FavoriteMessage>, String> {
    state
        .db
        .get_messages_by_tag(tag_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_tag_from_message(
    state: State<'_, AppState>,
    message_id: usize,
    tag_id: i64,
) -> Result<(), String> {
    state
        .db
        .remove_tag_from_message(message_id, tag_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_favorited_messages_with_tags(
    state: State<'_, AppState>,
) -> Result<Vec<(FavoriteMessage, Vec<Tag>)>, String> {
    state
        .db
        .get_favorited_messages_with_tags()
        .map_err(|e| e.to_string())
}
