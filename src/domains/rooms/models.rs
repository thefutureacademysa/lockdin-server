use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub grade: i32,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<i32>,
    pub is_live: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RoomRecord {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub grade: i32,
    pub category: String,
    pub is_live: bool,
    pub created_at: DateTime<Utc>,
}

pub enum SubjectCategory {
    STEM,
    HUMANITIES,
    COMMERCE,
}

impl SubjectCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "stem" => Some(SubjectCategory::STEM),
            "humanities" => Some(SubjectCategory::HUMANITIES),
            "commerce" => Some(SubjectCategory::COMMERCE),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            SubjectCategory::STEM => "STEM",
            SubjectCategory::HUMANITIES => "Humanities",
            SubjectCategory::COMMERCE => "Commerce",
        }
    }
}