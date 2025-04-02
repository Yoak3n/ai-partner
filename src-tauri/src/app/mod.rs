mod hotkey;
mod interaction;
mod invoke;
mod state;
mod updater;
mod window;

use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use tauri::{generate_handler, AppHandle};
use tauri_plugin_notification::NotificationExt;
// use tauri_plugin_window_state;
use updater::PendingUpdate;

use crate::utils;

use super::store::{db::Database, setting::Configuration};
use hotkey::register_shortcut;
use state::AppState;
// Global AppHandle
pub static APP: OnceCell<AppHandle> = OnceCell::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = Configuration::init_config().unwrap();
    let db = Database::new(".".into()).expect("Failed to initialize database");
    let app_state = AppState {
        config: Arc::new(Mutex::new(config)),
        db: Arc::new(db),
        pending_update: Arc::new(Mutex::new(PendingUpdate::new())),
    };
    let instance = tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            app.notification()
                .builder()
                .title("The program is already running. Please do not start it again!")
                .icon("ai-partner")
                .show()
                .unwrap();
        }))
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // OS
        .invoke_handler(generate_handler![
            // App
            #[cfg(desktop)]
            invoke::operation::create_dialog,
            #[cfg(desktop)]
            invoke::operation::get_app_install_path,
            #[cfg(desktop)]
            invoke::operation::register_shortcut_by_frontend,
            invoke::operation::set_config,
            invoke::operation::get_config,
            // Chat
            invoke::chat::completions_stream,
            invoke::chat::pause_stream,
            invoke::chat::create_conversation,
            invoke::chat::get_conversations,
            invoke::chat::save_message,
            invoke::chat::get_conversation_messages,
            invoke::chat::delete_conversation,
            invoke::favorite::add_new_favorite,
            invoke::favorite::remove_favorite,
            invoke::favorite::get_favorites,
            // Updater
            #[cfg(desktop)]
            invoke::update::fetch_update,
            #[cfg(desktop)]
            invoke::update::install_update,
            // Tag
            invoke::tag::add_tag,
            invoke::tag::get_tags,
            invoke::tag::add_tag_to_message,
            invoke::tag::get_message_tags,
            invoke::tag::get_messages_by_tag,
            invoke::tag::remove_tag_from_message,
            invoke::tag::get_favorited_messages_with_tags
        ])
        .setup(|app| {
            #[cfg(desktop)]
            {
                let app_handle = app.handle().clone();
                interaction::create_systray(app)?;
                app_handle.plugin(tauri_plugin_updater::Builder::new().build())?;
                APP.get_or_init(|| app.handle().clone());

                match register_shortcut("all") {
                    Ok(_) => {}
                    Err(_) => {
                        app.handle()
                            .notification()
                            .builder()
                            .title("Failed to register global shortcut")
                            .icon("ai-partner")
                            .show()
                            .unwrap_or_else(|e| 
                                println!("Failed to show notification: {}", e)
                            );
                    }
                }
                utils::timer::Timer::global()
                    .init()
                    .unwrap_or_else(|e| println!("Failed to init timer: {}", e));
                window::enable_auto_light_weight_mode();

                tauri::async_runtime::block_on(async move {
                    let _ = window::switch_main_window();
                });
            }
            Ok(())
        });

    let app = instance.build(tauri::generate_context!()).unwrap();
    app
        .run(|_,_|{});
}
