#[derive(Debug, serde::Serialize)]
pub struct Conversation {
    pub id: i64,
    pub title: String,
    pub created_at: String,
}

pub struct Faverite {
    pub id: i64,
    pub message_id: String,
    pub created_at: String,
}