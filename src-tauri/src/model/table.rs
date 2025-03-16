use serde::{Deserialize,Serialize};
#[derive(Debug,Serialize)]
pub struct Conversation {
    pub id: i64,
    pub title: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MessageTag {
    pub message_id: i64,
    pub tag_id: i64,
}
