use redis::RedisResult;
use crate::domains::auth::error::AppError;
use crate::domains::rooms::models::{Room, RoomRecord, SubjectCategory};

#[async_trait::async_trait]
pub trait RoomRepository {
    async fn get_all_rooms(
        &self,
        search: Option<String>,
        grade: Option<i32>,
        category: Option<SubjectCategory>,
    ) -> sqlx::Result<Vec<RoomRecord>, sqlx::Error>;
}

#[async_trait::async_trait]
pub trait PresenceRepository {
    // implement by redis
    async fn get_participant_count(&self, room_id: &str) -> RedisResult<usize>;
    async fn add_participant(&self, room_id: &str, user_id: &str) -> RedisResult<usize>;
    async fn remove_participant(&self, room_id: &str, user_id: &str) -> RedisResult<usize>;
}