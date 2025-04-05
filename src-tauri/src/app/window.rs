use delay_timer::prelude::TaskBuilder;
use tauri::{Error, Listener,Manager, WebviewUrl, WebviewWindowBuilder};
use anyhow::Result;
use super::APP;
use crate::utils::timer::Timer;

const LIGHT_WEIGHT_TASK_UID: &str = "light_weight_task";

pub fn switch_dialog_window() -> Result<(), Error> {
    let app = APP.get().unwrap();
    // let app_handle = app.clone();
    match app.get_webview_window("dialog") {
        Some(w) => {
            if w.is_visible()? {
                w.set_always_on_top(false)?;
                w.close()?;
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
            println!("create main window");
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

pub fn entry_lightweight_mode() {
    if let Some(window) = APP.get().unwrap().app_handle().get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        }
        if let Some(webview) = window.get_webview_window("main") {
            println!("entry light weight mode");
            let _ = webview.destroy();
        }
    }
    let _ = cancel_light_weight_timer();
}

pub fn enable_auto_light_weight_mode() {
    println!("enable auto light weight mode");
    setup_window_close_listener();
    setup_webview_focus_listener();
}

fn setup_window_close_listener() -> u32 {
    if let Some(window) = APP.get().unwrap().get_webview_window("main") {
        let handler = window.listen("tauri://close-requested", move |_event| {
            let _ = setup_light_weight_timer();
            println!("close main window");
        });
        return handler;
    }
    0
}

fn setup_webview_focus_listener() -> u32 {
    if let Some(window) = APP.get().unwrap().app_handle().get_webview_window("main")  {
        let handler = window.listen("tauri://focus", move |_event| {
            println!("focus");
            let _ = cancel_light_weight_timer();

        });
        return handler;
    }
    if let Some(window) = APP.get().unwrap().app_handle().get_webview_window("dialog")  {
        let handler = window.listen("tauri://focus", move |_event| {
            println!("focus");
            let _ = cancel_light_weight_timer();

        });
        return handler;
    }
    0
}

fn setup_light_weight_timer() -> Result<()> {
    Timer::global().init()?;

    let mut timer_map = Timer::global().timer_map.write().unwrap();
    let delay_timer = Timer::global().delay_timer.write();
    let mut timer_count = Timer::global().timer_count.lock().unwrap();

    let task_id = *timer_count;
    *timer_count += 1;


    let task = TaskBuilder::default()
        .set_task_id(task_id)
        .set_maximum_parallel_runnable_num(1)
        .set_frequency_once_by_minutes(5)
        .spawn_async_routine(move || async move {
            entry_lightweight_mode();
        })?;

    delay_timer
        .unwrap()
        .add_task(task)?;

    let timer_task = crate::utils::timer::TimerTask {
        task_id,
        interval_minutes: 5,
        last_run: chrono::Local::now().timestamp(),
    };

    timer_map.insert(LIGHT_WEIGHT_TASK_UID.to_string(), timer_task);

    Ok(())
}


fn cancel_light_weight_timer() -> Result<()> {
    let mut timer_map = Timer::global().timer_map.write().unwrap();
    let delay_timer = Timer::global().delay_timer.write().unwrap();

    if let Some(task) = timer_map.remove(LIGHT_WEIGHT_TASK_UID) {
        delay_timer
            .remove_task(task.task_id)?;
    }

    Ok(())
}
