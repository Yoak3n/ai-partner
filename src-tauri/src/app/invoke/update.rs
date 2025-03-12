use super::super::updater::{PendingUpdate,UpdateMetadata,Result,DownloadEvent, UpdaterError};
use tauri::{ipc::Channel, State, AppHandle};
use tauri_plugin_updater::UpdaterExt;
use super::super::state::AppState;
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
        //   .endpoints(vec![url])?
        .build()?
        .check()
        .await?;

    let update_metadata = update.as_ref().map(|update| UpdateMetadata {
        version: update.version.clone(),
        current_version: update.current_version.clone(),
    });
    let pending_update = state.pending_update.lock().unwrap();
    // let Some(mut pending_update) =  state.pending_update.lock().unwrap().0.lock().unwrap().take() else{
    //     return Err(UpdaterError::NoPendingUpdate);
    // };
    *pending_update.0.lock().unwrap() = update;

  Ok(update_metadata)
}

#[tauri::command]
pub async fn install_update(state: State<'_, AppState>, on_event: Channel<DownloadEvent>) -> Result<()> {
    let Some(pending_update) =  state.pending_update.lock().unwrap().0.lock().unwrap().take() else{
        return Err(UpdaterError::NoPendingUpdate);
    };
    // let Some(update) = pending_update.0.lock().unwrap().take() else {
    //     return Err(UpdaterError::NoPendingUpdate);
    // };

    let mut started = false;

    pending_update
        .download_and_install(
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

    Ok(())
}