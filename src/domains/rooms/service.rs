use crate::domains::rooms::models::{Room, SubjectCategory};
use crate::domains::rooms::repository::RoomRepository;
use std::io::Error;
use std::sync::Arc;

pub struct RoomsService {
    pub repo: Arc<dyn RoomRepository + Sync + Send>,
}

impl RoomsService {
    pub async fn get_all_rooms(
        &self,
        grade: Option<i32>,
        category: Option<String>,
    ) -> Result<Vec<Room>, Error> {
        let category = SubjectCategory::from_str(category.as_deref().unwrap_or(""));
        match self.repo.get_all_rooms(grade.clone(), category).await {
            Ok(rooms) => Ok(rooms),
            Err(e) => {
                log::error!("Error getting all rooms: {}", e);
                Err(Error::other(e.to_string()))
            }
        }
    }
}
