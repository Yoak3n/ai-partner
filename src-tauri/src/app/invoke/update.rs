use super::super::state::AppState;
use super::super::updater::{DownloadEvent, Result, UpdateMetadata, UpdaterError};
use tauri::{ipc::Channel, AppHandle, Manager, State};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_updater::UpdaterExt;

#[tauri::command]
pub async fn fetch_update(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Option<UpdateMetadata>> {
    // let channel = "stable";
    // let url = Url::parse(&format!(
    //     "https://cdn.myupdater.com/{{{{target}}}}-{{{{arch}}}}/{{{{current_version}}}}?channel={channel}",
    // )).expect("invalid URL");
    let update = app
        .updater_builder()
        .timeout(std::time::Duration::from_secs(30))
        // 目前这样挺好的。默认使用的版本号比较器与当前的版本号命名规则不适配，patch字段单独比较大小会导致版本号比较错误，如0.1.7 低于 0.1.61导致不能更新到0.1.7
        .version_comparator(|c, r| c != r.version)
        //   .endpoints(vec![url])?
        .build()?
        .check()
        .await?;
    if let Some(_) = update {
        app.app_handle()
            .notification()
            .builder()
            .title("Found pending update")
            .icon("ai-partner")
            .show()
            .unwrap_or_else(|e| println!("Failed to show notification: {}", e));
    } else {
        return Err(UpdaterError::NoPendingUpdate);
    }
    let update_metadata = update
        .as_ref()
        .map(|update| 
            UpdateMetadata {
                version: update.version.clone(),
                current_version: update.current_version.clone(),
                note: update.body
                    .as_ref()
                    .map(|note| note.clone())
                    .unwrap_or_default(),
            });
    let pending_update = state.pending_update.lock().unwrap();
    *pending_update.0.lock().unwrap() = update;

    Ok(update_metadata)
}

#[tauri::command]
pub async fn install_update(
    state: State<'_, AppState>,
    on_event: Channel<DownloadEvent>,
) -> Result<()> {
    let Some(pending_update) = state
        .pending_update
        .lock()
        .unwrap()
        .0
        .lock()
        .unwrap()
        .take()
    else {
        return Err(UpdaterError::NoPendingUpdate);
    };

    let mut started = false;
    let buf = pending_update
        .download(
            |chunk_length, content_length| {
                if !started {
                    let _ = on_event.send(DownloadEvent::Started { content_length });
                    started = true;
                }
                let _ = on_event.send(DownloadEvent::Progress { chunk_length });
            },
            || {
                let _ = on_event.send(DownloadEvent::Finished);
            },
        )
        .await?;
    pending_update.install(buf)?;

    Ok(())
}
