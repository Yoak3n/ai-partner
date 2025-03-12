mod invoke;
mod state;
mod interaction;
mod hotkey;
mod window;
mod updater;

use tauri::AppHandle;
use updater::PendingUpdate;
use std::sync::{Mutex,Arc};
use once_cell::sync::OnceCell;
use tauri_plugin_window_state;
use tauri_plugin_notification::NotificationExt;

use super::store::{setting::Configuration,db::Database};
use state::AppState;
use hotkey::register_shortcut;
// Global AppHandle
pub static APP: OnceCell<AppHandle> = OnceCell::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let config = Configuration::init_config().unwrap();
    let db = Database::new(".".into()).expect("Failed to initialize database");
    let app_state = AppState{
        config:Arc::new(Mutex::new(config)),
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
        .invoke_handler(tauri::generate_handler![
            invoke::operation::create_dialog,
            invoke::operation::get_app_install_path,
            invoke::operation::register_shortcut_by_frontend,
            invoke::operation::set_config,
            invoke::operation::get_config,
            invoke::chat::completions_stream,
            invoke::chat::pause_stream,
            invoke::chat::create_conversation,
            invoke::chat::get_conversations,
            invoke::chat::save_message,
            invoke::chat::get_conversation_messages,
            invoke::chat::delete_conversation,
            #[cfg(desktop)]
            invoke::update::fetch_update,
            #[cfg(desktop)]
            invoke::update::install_update,
            ]
        );
       
        instance
        .setup(|app| {

            #[cfg(desktop)]
            {  
                let app_handle = app.handle().clone();
                // interaction::register_shortcuts(app)?;
                interaction::create_systray(app)?;
                app_handle.plugin(tauri_plugin_updater::Builder::new().build())?;
                tauri::async_runtime::spawn(async move {
                    updater::update(app_handle).await.unwrap();
                  });
                APP.get_or_init(|| app.handle().clone());
                  
                match register_shortcut("all") {
                    Ok(()) => println!("Successfully registered shortcuts"),
                    Err(e) => {
                        println!("Failed to register shortcuts: {}", e);
                        app.handle().notification().builder()
                            .title("Failed to register global shortcut")
                            .body(&e)
                            .icon("ai-partner")
                            .show()
                            .unwrap_or_else(|e| println!("Failed to show notification: {}", e));
                    }
                }
            }

            

            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
