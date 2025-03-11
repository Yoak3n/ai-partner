#[derive(Debug, serde::Serialize)]
pub struct Conversation {
    pub id: i64,
    pub title: String,
    pub created_at: String,
}