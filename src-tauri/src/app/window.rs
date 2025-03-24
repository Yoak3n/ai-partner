use super::APP;
use tauri::{Error, Manager, WebviewUrl, WebviewWindowBuilder};
pub fn switch_dialog_window() -> Result<(), Error> {
    let app = APP.get().unwrap();
    // let app_handle = app.clone();
    match app.get_webview_window("dialog") {
        Some(w) => {
            if w.is_visible()? {
                w.set_always_on_top(false)?;
                w.hide()?;
            } else {
                w.set_focus()?;
                w.set_always_on_top(true)?;
                w.show()?;
            }
        }
        None => {
            // 手动创建窗口会导致程序无响应，需要改用自动创建，这段代码暂时保留，以后再研究
            // 问题出在缓存窗口状态的插件，修改后会导致之前的窗口状态丢失从而导致无响应
            println!("create dialog window");
            WebviewWindowBuilder::new(app, "dialog", WebviewUrl::App("/dialog".into()))
                .transparent(true)
                .title("")
                .resizable(false)
                .decorations(false)
                .shadow(false)
                .inner_size(800.0, 100.0)
                .center()
                .build()?;
        }
    }
    Ok(())
}


pub fn switch_main_window() -> Result<(), Error> {
    let app = APP.get().unwrap();
    match app.get_webview_window("main"){
        Some(w) => {
            if w.is_visible()? {
                w.set_always_on_top(false)?;
                w.hide()?;
            }else{
                w.set_focus()?;
                w.show()?;
            }
        }
        None => {
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("/".into()))
                .title("ai-partner")
                .resizable(true)
                .decorations(false)
                .inner_size(1024.0, 720.0)
                .min_inner_size(600.0, 600.0)
                .center()
                .build()?;
        }
    }
    Ok(())
}