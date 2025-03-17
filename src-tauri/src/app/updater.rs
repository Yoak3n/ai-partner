use crate::store::setting::get;
use tauri::{AppHandle, Manager, Url};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_updater::{Update, UpdaterExt};

#[allow(dead_code)]
pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    let proxy_url = if let Some(proxy) = get("proxy") {
        if proxy.is_empty() {
            None
        } else {
            Some(Url::parse(proxy.as_str()).unwrap())
        }
    } else {
        None
    };
    match update_process(&app, proxy_url).await {
        Some(update) => {
            let mut downloaded = 0;
            update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length;
                        println!("downloaded {downloaded} from {content_length:?}");
                    },
                    || {
                        println!("download finished");
                    },
                )
                .await?;
            println!("update installed");
            app.app_handle()
                .notification()
                .builder()
                .title("update installed")
                .icon("ai-partner")
                .show()
                .unwrap();
            app.restart();
        }
        None => {
            println!("Failed to check for updates");
            app.app_handle()
                .notification()
                .builder()
                .title("Failed to check for updates")
                .icon("ai-partner")
                .show()
                .unwrap();
        }
    }
    Ok(())
}
#[allow(dead_code)]
async fn update_process(app: &AppHandle, proxy: Option<Url>) -> Option<Update> {
    match proxy {
        Some(proxy_url) => app
            .updater_builder()
            .timeout(std::time::Duration::from_secs(30))
            .proxy(proxy_url)
            .build()
            .unwrap()
            .check()
            .await
            .unwrap(),
        None => app
            .updater_builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap()
            .check()
            .await
            .unwrap(),
    }
}
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum UpdaterError {
    #[error(transparent)]
    Updater(#[from] tauri_plugin_updater::Error),
    #[error("there is no pending update")]
    NoPendingUpdate,
}

impl Serialize for UpdaterError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum DownloadEvent {
    #[serde(rename_all = "camelCase")]
    Started { content_length: Option<u64> },
    #[serde(rename_all = "camelCase")]
    Progress { chunk_length: usize },
    #[serde(rename_all = "camelCase")]
    Finished,
}

use std::sync::Mutex;

pub type Result<T> = std::result::Result<T, UpdaterError>;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMetadata {
    pub version: String,
    pub current_version: String,
    pub note: String,
}

pub struct PendingUpdate(pub Mutex<Option<Update>>);

impl PendingUpdate {
    pub fn new() -> Self {
        Self(Mutex::new(None))
    }
}
