use super::APP;
use tauri::{Error, Manager, WebviewUrl, WebviewWindowBuilder};
pub fn switch_dialog_window() -> Result<(), Error> {
    let app = APP.get().unwrap();
    // let app_handle = app.clone();
    match app.get_webview_window("dialog") {
        Some(w) => {
            let v = w.is_visible()?;
            if v {
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
            println!("create dialog window");
            WebviewWindowBuilder::new(app, "dialog", WebviewUrl::App("/dialog".into()))
                .transparent(true)
                .title("")
                .resizable(false)
                .decorations(false)
                .build()?;
        }
    }
    Ok(())
}
