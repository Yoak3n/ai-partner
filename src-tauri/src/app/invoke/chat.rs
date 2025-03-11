use futures::StreamExt;
use reqwest::{Client,header::{HeaderMap,HeaderValue,AUTHORIZATION}};
use serde_json::json;
use super::super::state::AppState;
use tauri::Emitter;
#[tauri::command]
pub async fn completions_stream(app_handle: tauri::AppHandle, state: State<'_,AppState>,id: usize,messages:Vec<MessageItem>) -> Result<MessageItem, String> {
    let api = state.config.try_lock().expect("get config of state error").api.clone();
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", &api.key))
            .expect("Invalid authorization header")
    );
    let payload = json!({
        "stream": true,
        "messages": messages,
        "model":&api.model,
    });
    // 处理不完整Url输入
    let url = if api.url.ends_with("v1/") {
        format!("{}chat/completions", api.url)
    }else if api.url.ends_with("v1"){
        format!("{}/chat/completions", api.url)
    }else{
        api.url
    };
    let response = match client
        .post(&url)
        .headers(headers)
        .json(&payload)
        .send()
        .await {
            Ok(res) => res,
            Err(e) => {
                println!("请求失败: {}", e);
                let payload = StreamError::new(e.to_string(), id);
                let _ = app_handle.emit_to("main","stream-error",&payload);
                return Err(e.to_string());
            }
        };
    if response.status() != 200 {
        println!("请求失败: {}", response.status());
        let payload = StreamError::new(response.status().to_string(), id);
        let _ = app_handle.emit_to("main","stream-error",&payload);
        return Err(response.status().to_string());
    }
    let mut full = MessageItem::default();
    full.timestamp = id; // 设置消息的时间戳为传入的id
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                if let Some(ret) = handle_stream_data(&bytes){
                    for item in ret{
                        full.append(&item);
                        let payload = StreamEmitter::new(item, 0, full.timestamp); // 使用消息时间戳作为id
                        if let Err(e) = app_handle.emit("stream-data", payload) {
                            eprintln!("事件发送失败: {}", e);
                        }
                    }
                }else{
                    let item = MessageType::DONE;
                    let payload = StreamEmitter::new(item, 0, full.timestamp);
                    if let Err(e) = app_handle.emit("stream-data", payload) {
                        eprintln!("事件发送失败: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("流错误: {}", e);
                let payload = StreamError::new(e.to_string(), full.timestamp);
                let _ = app_handle.emit_to("main","stream-error", &payload);
            }
        }
    }
    Ok(full)
}

use std::io::{BufReader,BufRead};
fn handle_stream_data(data: &[u8])->Option<Vec<MessageType>> {
    let mut ret:Vec<MessageType> = vec![];
    let reader = BufReader::new(data);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("data: ") {
            let content = line.trim_start_matches("data: ").trim();
            if content == "[DONE]" {
                return None;
            }
            // 解析JSON
            match serde_json::from_str::<StreamData>(content) {
                Ok(json) => {
                    if let Some(c) = &json.choices[0].delta.content{
                        if !c.is_empty(){ret.push(MessageType::Content(c.clone()));}
                    }else if let Some(r) = &json.choices[0].delta.reasoning_content{
                        if !r.is_empty(){ret.push(MessageType::ReasoningContent(r.clone()));}
                    }
                }
                Err(e) => {
                    eprintln!("JSON解析失败: {} | 原始内容: {}", e, content);
                    continue;
                }
            }
        };
    }
    Some(ret)
}


use crate::model::{table::Conversation, MessageItem, MessageType, StreamData, StreamEmitter, StreamError};


#[tauri::command]
pub async fn pause_stream(app: tauri::AppHandle,id:usize) -> Result<(), String> {
    let item = MessageType::DONE;
    let payload = StreamEmitter::new(item, 0, id);
    if let Err(e) = app.emit("stream-data", payload) {
        eprintln!("事件发送失败: {}", e);
    }
    Ok(())
}

use tauri::State;


#[tauri::command]
pub async fn create_conversation(state: State<'_, AppState>, title: String) -> Result<i64, String> {
    state.db.create_conversation(&title)
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn get_conversations(state: State<'_, AppState>) -> Result<Vec<Conversation>, String> {
    state.db.get_conversations()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_message(
    state: State<'_, AppState>, 
    conversation_id: i64,
    message: MessageItem
) -> Result<i64, String> {
    state.db.save_message(conversation_id, &message)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_conversation_messages(
    state: State<'_, AppState>,
    conversation_id: i64
) -> Result<Vec<MessageItem>, String> {
    state.db.get_conversation_messages(conversation_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_conversation(
    state: State<'_, AppState>,
    conversation_id: i64
) -> Result<(), String> {
    state.db.delete_conversation(conversation_id)
        .map_err(|e| e.to_string())
}