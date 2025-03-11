use tauri::{AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt,ShortcutState};

use crate::store::setting::{get,set};

use super::{APP,window::switch_dialog_window};


pub fn register<F,R:Runtime>(app_handle: &AppHandle<R>, name: &str, handler: F, key: &str) -> Result<(), String>
where
    F: Fn() + Send + Sync + 'static,
{
    let hotkey_name = format!("hotkey.{}",name);
    let hotkey = {
        if key.is_empty() {
            match get(hotkey_name.as_str()) {
                Some(v) => v.as_str().to_string(),
                None => {
                    set(hotkey_name.as_str(), "").unwrap();
                    String::new()
                }
            }
        } else {
            key.to_string()
        }
    };
    if !hotkey.is_empty() {
        let global_shortcut_manager = app_handle.global_shortcut();
        match global_shortcut_manager.on_shortcut(hotkey.as_str(),move |_, _, event| {
                if event.state == ShortcutState::Pressed {
                    handler();
                }
            }){
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to register global shortcut: {} {:?}", hotkey, e);
                return Err(e.to_string());
            }
        };
    }
    Ok(())
}

pub fn register_shortcut(shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match shortcut {
        "dialog" => register(
            app_handle,
            "dialog",
            || { let _ = switch_dialog_window(); },
            "",
        )?,
        "all" => {
            register(
                app_handle,
                "dialog",
                || { let _ = switch_dialog_window(); },
                "",
            )?;
        }
        _ => {}
    }
    Ok(())
}
