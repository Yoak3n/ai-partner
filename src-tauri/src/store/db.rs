use super::module::*;
use crate::model::{
    table::{Conversation, Tag},
    FavoriteMessage, MessageItem,
};
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::RwLock;

pub struct Database {
    conn: RwLock<Connection>,
}

impl Database {
    pub fn new(app_dir: PathBuf) -> Result<Self> {
        let conn = Connection::open(app_dir.join("chat_history.db"))?;

        // 创建 conversations 表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 创建 messages 表，添加 conversation_id 外键
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY,
                conversation_id INTEGER NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                reasoning_content TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(conversation_id) REFERENCES conversations(id)
            )",
            [],
        )?;

        // 考虑将favorites与conversation合并，当然，是在主对话也支持收藏之后
        conn.execute(
            "CREATE TABLE IF NOT EXISTS favorites (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                model TEXT NOT NULL,
                message_id INTEGER NOT NULL UNIQUE,
                reasoning_content TEXT,
                content TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                color TEXT NOT NULL DEFAULT '#1890ff',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        // 创建消息-标签关联表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS message_tags (
                message_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (message_id, tag_id),
                FOREIGN KEY(message_id) REFERENCES favorites(message_id),
                FOREIGN KEY(tag_id) REFERENCES tags(id)
            )",
            [],
        )?;

        Ok(Database {
            conn: RwLock::new(conn),
        })
    }
}

// 实现会话管理 trait
impl ConversationManager for Database {
    fn create_conversation(&self, title: &str) -> Result<i64> {
        let conn: std::sync::RwLockWriteGuard<'_, Connection> = self.conn.write().unwrap();
        conn.execute("INSERT INTO conversations (title) VALUES (?1)", [title])?;
        Ok(conn.last_insert_rowid())
    }

    fn get_conversations(&self) -> Result<Vec<Conversation>> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, title, created_at FROM conversations ORDER BY created_at DESC")?;

        let conversations = stmt.query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;

        conversations.collect()
    }

    fn delete_conversation(&self, conversation_id: i64) -> Result<()> {
        let mut conn = self.conn.write().unwrap();
        // 开始事务
        let tx = conn.transaction()?;

        // 先删除对话相关的所有消息
        tx.execute(
            "DELETE FROM messages WHERE conversation_id = ?",
            [conversation_id],
        )?;

        // 再删除对话本身
        tx.execute("DELETE FROM conversations WHERE id = ?", [conversation_id])?;

        // 提交事务
        tx.commit()?;
        Ok(())
    }
}
// 实现消息管理 trait
impl MessageManager for Database {
    fn save_message(&self, conversation_id: i64, message: &MessageItem) -> Result<i64> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "INSERT INTO messages (
                id,
                conversation_id, 
                role, 
                content, 
                reasoning_content) 
                VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                message.timestamp,
                conversation_id,
                &message.role,
                &message.content,
                &message.reasoning_content,
            ),
        )?;
        Ok(conn.last_insert_rowid())
    }

    fn get_conversation_messages(&self, conversation_id: i64) -> Result<Vec<MessageItem>> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn.prepare(
            "SELECT role, content, reasoning_content ,id
             FROM messages 
             WHERE conversation_id = ?1 
             ORDER BY created_at ASC",
        )?;

        let messages = stmt.query_map([conversation_id], |row| {
            Ok(MessageItem {
                role: row.get(0)?,
                content: row.get(1)?,
                reasoning_content: row.get(2)?,
                timestamp: row.get(3)?,
                // 通过对话获取消息不需要favorite字段
            })
        })?;

        messages.collect()
    }
}
// 实现收藏管理 trait
impl FavoriteManager for Database {
    fn favorite_message(&self, message: &MessageItem, model: String) -> Result<i64> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO favorites (
                message_id,
                reasoning_content,
                content,
                model
            ) VALUES (?1,?2,?3,?4)",
            (
                message.timestamp,
                &message.reasoning_content,
                &message.content,
                model,
            ),
        )?;
        Ok(conn.last_insert_rowid())
    }

    fn unfavorite_message(&self, message_id: i64) -> Result<()> {
        let conn = self.conn.write().unwrap();
        conn.execute("DELETE FROM favorites WHERE message_id = ?", [message_id])?;
        Ok(())
    }

    fn get_favorited_messages(&self) -> Result<Vec<FavoriteMessage>> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn.prepare(
            "SELECT 
                id, message_id, content, reasoning_content, model
                FROM favorites
                ORDER BY created_at DESC",
        )?;
        //  let mut stmt = conn.prepare(
        //     "SELECT
        //         m.role,
        //         m.content,
        //         m.reasoning_content,
        //         m.id AS message_id,
        //         f.id AS favorite_id,
        //         f.created_at AS favorite_time
        //     FROM
        //         messages m
        //     INNER JOIN
        //         favorites f
        //     ON
        //         m.id = f.message_id
        //     ORDER BY
        //         f.created_at DESC;"
        // )?;
        let messages = stmt.query_map([], |row| {
            Ok(FavoriteMessage {
                id: row.get(0)?,
                message_id: row.get(1)?,
                content: row.get(2)?,
                reasoning_content: row.get(3)?,
                model: row.get(4)?,
            })
        })?;
        messages.collect()
    }
}
// 实现标签管理 trait
impl TagManager for Database {
    fn create_tag(&self, name: &str, color: &str) -> Result<i64> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "INSERT INTO tags (name, color) VALUES (?1, ?2)",
            [name, color],
        )?;
        Ok(conn.last_insert_rowid())
    }

    fn get_tags(&self) -> Result<Vec<Tag>> {
        let conn = self.conn.read().unwrap();
        let mut stmt =
            conn.prepare("SELECT id, name, color, created_at FROM tags ORDER BY name")?;

        let tags = stmt.query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;

        tags.collect()
    }

    fn add_tag_to_message(&self, message_id: usize, tag_id: i64) -> Result<()> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO message_tags (message_id, tag_id) VALUES (?1, ?2)",
            [message_id, tag_id.try_into().unwrap()],
        )?;
        Ok(())
    }

    fn remove_tag_from_message(&self, message_id: usize, tag_id: i64) -> Result<()> {
        let conn = self.conn.write().unwrap();
        conn.execute(
            "DELETE FROM message_tags WHERE message_id = ?1 AND tag_id = ?2",
            [message_id, tag_id.try_into().unwrap()],
        )?;
        Ok(())
    }

    fn get_message_tags(&self, message_id: usize) -> Result<Vec<Tag>> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.color, t.created_at 
             FROM tags t
             JOIN message_tags mt ON t.id = mt.tag_id
             WHERE mt.message_id = ?1
             ORDER BY t.name",
        )?;

        let tags = stmt.query_map([message_id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;

        tags.collect()
    }

    fn get_messages_by_tag(&self, tag_id: i64) -> Result<Vec<FavoriteMessage>> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn.prepare(
            "SELECT f.id, f.message_id, f.content, f.reasoning_content, f.model
             FROM favorites f
             JOIN message_tags mt ON f.message_id = mt.message_id
             WHERE mt.tag_id = ?1
             ORDER BY f.created_at DESC",
        )?;

        let messages = stmt.query_map([tag_id], |row| {
            Ok(FavoriteMessage {
                id: row.get(0)?,
                message_id: row.get(1)?,
                content: row.get(2)?,
                reasoning_content: row.get(3)?,
                model: row.get(4)?,
            })
        })?;

        messages.collect()
    }

    fn get_favorited_messages_with_tags(&self) -> Result<Vec<(FavoriteMessage, Vec<Tag>)>> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn.prepare(
            "SELECT 
                id, message_id, content, reasoning_content, model
                FROM favorites
                ORDER BY created_at DESC",
        )?;

        let messages = stmt.query_map([], |row| {
            let message = FavoriteMessage {
                id: row.get(0)?,
                message_id: row.get(1)?,
                content: row.get(2)?,
                reasoning_content: row.get(3)?,
                model: row.get(4)?,
            };

            Ok(message)
        })?;

        let mut result = Vec::new();
        for message_result in messages {
            let message = message_result?;
            let tags = self.get_message_tags(message.message_id)?;
            result.push((message, tags));
        }

        Ok(result)
    }
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}
