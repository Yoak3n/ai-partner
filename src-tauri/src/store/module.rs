use crate::model::{
    table::{Conversation, Tag},
    FavoriteMessage, MessageItem,
};
use rusqlite::Error;
pub trait ConversationManager {
    fn create_conversation(&self, title: &str) -> Result<i64, Error>;
    fn get_conversations(&self) -> Result<Vec<Conversation>, Error>;
    fn delete_conversation(&self, conversation_id: i64) -> Result<(), Error>;
}

// 消息管理相关的 trait
pub trait MessageManager {
    fn save_message(&self, conversation_id: i64, message: &MessageItem) -> Result<i64, Error>;
    fn get_conversation_messages(&self, conversation_id: i64) -> Result<Vec<MessageItem>, Error>;
}

// 收藏管理相关的 trait
pub trait FavoriteManager {
    fn favorite_message(&self, message: &MessageItem, model: String) -> Result<i64, Error>;
    fn unfavorite_message(&self, message_id: i64) -> Result<(), Error>;
    fn get_favorited_messages(&self) -> Result<Vec<FavoriteMessage>, Error>;
}

// 标签管理相关的 trait
pub trait TagManager {
    fn create_tag(&self, name: &str, color: &str) -> Result<i64, Error>;
    fn get_tags(&self) -> Result<Vec<Tag>, Error>;
    fn add_tag_to_message(&self, message_id: usize, tag_id: i64) -> Result<(), Error>;
    fn remove_tag_from_message(&self, message_id: usize, tag_id: i64) -> Result<(), Error>;
    fn get_message_tags(&self, message_id: usize) -> Result<Vec<Tag>, Error>;
    fn get_messages_by_tag(&self, tag_id: i64) -> Result<Vec<FavoriteMessage>, Error>;
    fn get_favorited_messages_with_tags(&self) -> Result<Vec<(FavoriteMessage, Vec<Tag>)>, Error>;
}
