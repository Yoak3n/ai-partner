use tauri::{
    Error,
    Manager,
    WebviewWindowBuilder,WebviewUrl,
    PhysicalSize
};
use super::APP;


pub fn switch_dialog_window()-> Result<(), Error> {
    let app = APP.get().unwrap();
    let app_handle = app.clone();
    match app.get_webview_window("dialog") {
        Some(w) => {
            let v = w.is_visible()?;
            if v {
                w.set_always_on_top(false)?;
                w.hide()?;
            }else{
                w.set_focus()?;
                w.set_always_on_top(true)?;
                w.show()?;
            }
        },
        None => {
            let window = WebviewWindowBuilder::new(
                &app_handle ,"dialog", 
                WebviewUrl::App("/dialog".into()))
                .transparent(true)
                .center()
                .title("")
                .resizable(false)
                .shadow(false)
                .always_on_top(true)
                .decorations(false)
                .build()?;
            window.set_size(PhysicalSize::new(600, 400))?;
        } 
    }
    Ok(())
}
