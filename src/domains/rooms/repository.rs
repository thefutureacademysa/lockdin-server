use crate::domains::rooms::models::{Room, SubjectCategory};

#[async_trait::async_trait]
pub trait RoomRepository {
    async fn get_all_rooms(
        &self,
        search: Option<String>,
        grade: Option<i32>,
        category: Option<SubjectCategory>,
    ) -> sqlx::Result<Vec<Room>, sqlx::Error>;
}