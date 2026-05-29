use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub grade: String,
    pub category: String,
    pub participant_count: i32,
    pub is_live: bool,
    pub created_at: String,
}