use tauri::{State, Error};
use super::super::{
    APP,Configuration,AppState,
    window::switch_dialog_window,
    hotkey::register,
};

#[tauri::command]
pub async fn create_dialog() -> Result<(), Error> {
    switch_dialog_window()
}


use std::env::current_exe;
#[tauri::command]
pub fn get_app_install_path() -> Result<String, String> {
    let exe_path = current_exe().map_err(|e| e.to_string())?;
    
    let install_dir = if cfg!(target_os = "macos") {
        // macOS: 可执行文件位于 .app/Contents/MacOS/ 下
        exe_path.parent()  // MacOS 目录
            .and_then(|p| p.parent())  // Contents 目录
            .and_then(|p| p.parent())  // .app 目录
    } else {
        // Windows 和 Linux: 可执行文件在安装目录的子目录或根目录
        exe_path.parent()
    };
    
    install_dir
        .ok_or("无法确定安装目录".to_string())
        .and_then(|p| p.to_str()
            .ok_or("路径转换错误".to_string())
            .map(|s| s.to_string()))
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Configuration, String> {
    let config =  state.config.try_lock().expect("get config lock error");
    Ok(config.clone())
}

#[tauri::command]
pub async fn set_config(state : State<'_,AppState>,new_config: Configuration) -> Result<(), String> {
    // update_config(|c|{
    if let Err(e) = Configuration::update_config(move|config|
        {
            config.api.key = new_config.api.key.clone();
            config.api.url = new_config.api.url.clone();
            config.api.model = new_config.api.model.clone();
            config.hotkey.dialog = new_config.hotkey.dialog.clone();
            let mut c = state.config.try_lock().expect("set config lock error");
            *c = config.clone();
        })
    {
        return Err(e.to_string());
    }
    // });
    Ok(())
}

#[tauri::command]
pub fn register_shortcut_by_frontend(name: &str, shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match name {
        "dialog" => register(
            app_handle,
            "dialog",
            ||{let _ = switch_dialog_window();},
            shortcut,
        )?,
        _ => {}
    }
    Ok(())
}