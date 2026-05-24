use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub full_name: String,
    pub school_name: String,
    pub grade: String,
    pub email: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        full_name: String,
        school_name: String,
        grade: String,
        email: String,
    ) -> Self {
        // generate id
        let id = uuid::Uuid::now_v7().to_string();
        Self {
            id,
            full_name,
            school_name,
            grade,
            email,
            avatar_url: None,
            is_verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
