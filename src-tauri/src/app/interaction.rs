use tauri::{
    menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder}, Manager
};
use tauri::{App, Emitter, Error};
use tauri_plugin_global_shortcut::{Builder, Code, Modifiers, ShortcutState};

#[allow(dead_code)]
pub fn register_shortcuts(app: &App) -> Result<(), Error> {
    let handle = app.handle();
    handle.plugin({
        if let Ok(builder) = Builder::new().with_shortcuts(["alt+n", "alt+1"]) {
            builder
                .with_handler(|app_handle, shortcut, event| {
                    let main_window = app_handle.get_webview_window("main").unwrap();
                    if event.state == ShortcutState::Pressed {
                        if shortcut.matches(Modifiers::ALT, Code::KeyN) {
                            let _ = main_window.emit("dialog", "1");
                        }
                        if shortcut.matches(Modifiers::ALT, Code::Digit1) {
                            let _ = main_window.emit("dialog", "1");
                        }
                    }
                })
                .build()
        } else {
            Builder::new().build()
        }
    })
}


pub fn create_systray(app: &mut App) -> Result<(), Error> {
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "GUI", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&hide_i, &quit_i])?;
    let hide_i_clone = hide_i.clone();
    let _ = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("ai-partner")
        .on_tray_icon_event(move |tray_handle, event| {
            let app_handle = tray_handle.app_handle();
            // 检查是否是左键点击事件
            match event {
                tauri::tray::TrayIconEvent::Click { button, button_state, .. } => {
                    if button_state == MouseButtonState::Down{
                        match button {
                            MouseButton::Right => {
                                if let Some(main_window) = app_handle.get_webview_window("main") {
                                    // 获取窗口状态
                                    if let Ok(is_visible) = main_window.is_visible() {
                                        if is_visible {let _ = hide_i_clone.set_text("隐藏");
                                        } else {let _ = hide_i_clone.set_text("显示");}
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
                tauri::tray::TrayIconEvent::DoubleClick { button,.. } => {
                    if button == MouseButton::Left {
                        if let Some(main_window) = app_handle.get_webview_window("main") {
                            // 获取窗口状态
                            if let Ok(is_visible) = main_window.is_visible() {
                                if is_visible { let _ = main_window.hide();
                                } else {
                                    let _ = main_window.show();
                                    let _ = main_window.set_focus();
                                }
                            }
                        }
                    }
                }
                _ => {}
                
            }
        })
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            "hide" => {
                if let Some(main_window) = app.get_webview_window("main") {
                    if main_window.is_visible().unwrap() {
                        main_window.hide().unwrap();
                    } else {
                        main_window.show().unwrap();
                    }
                }
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;
    Ok(())
}
