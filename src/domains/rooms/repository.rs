use crate::domains::rooms::models::{Room, SubjectCategory};

#[async_trait::async_trait]
pub trait RoomRepository {
    async fn get_all_rooms(
        &self,
        grade: Option<i32>,
        category: Option<SubjectCategory>,
    ) -> sqlx::Result<Vec<Room>, sqlx::Error>;
}