#[allow(dead_code)]
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub sender_id: String,
    pub data: String,
    pub time_stamp: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum EventType {
    Local,
    Recieve,
    Send,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub r#type: EventType,
    pub message: Option<Message>
}