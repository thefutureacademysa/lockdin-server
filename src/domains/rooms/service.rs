use crate::domains::rooms::models::{Room, RoomRecord, SubjectCategory};
use crate::domains::rooms::repository::{PresenceRepository, RoomRepository};
use std::io::Error;
use std::sync::Arc;
use redis::RedisResult;
use crate::domains::auth::error::AppError;

pub struct RoomsService {
    pub repo: Arc<dyn RoomRepository + Sync + Send>,
    pub presence: Arc<dyn PresenceRepository + Sync + Send>,
}

impl RoomsService {
    pub async fn get_rooms(
        &self,
        search: Option<String>,
        grade: Option<i32>,
        category: Option<String>,
    ) -> Result<Vec<RoomRecord>, Error> {
        let category = SubjectCategory::from_str(category.as_deref().unwrap_or(""));
        match self.repo.get_all_rooms(search, grade, category).await {
            Ok(rooms) => Ok(rooms),
            Err(e) => {
                log::error!("Error getting all rooms: {}", e);
                Err(Error::other(e.to_string()))
            }
        }
    }

    pub async fn join_room(
        &self,
        room_id: &str,
        user_id: &str,
    ) -> Result<usize, AppError> {
        match self.presence.add_participant(room_id, user_id).await {
            Ok(result) => {
                log::info!("rooms_service: added participant {user_id} to room {room_id}");
                Ok(result)
            },
            Err(e) => {
                log::error!("rooms_service: failed adding participant {user_id} to room {room_id} - \n{e}");
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }
    
    pub async fn leave_room(
        &self,
        room_id: &str,
        user_id: &str,
    ) -> Result<usize, AppError> {
        match self.presence.remove_participant(room_id, user_id).await { 
            Ok(result) => {
                log::info!("rooms_service: removed participant {user_id} from room {room_id}");
                Ok(result)
            },
            Err(e) => {
                log::error!("rooms_service: failed removing participant {user_id} from room {room_id} - \n{e}");
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }

    pub async fn participants(
        &self,
        room_id: &str,
    ) -> Result<usize, AppError> {
        match self.presence.get_participant_count(room_id).await {
            Ok(result) => {
                log::info!("rooms_service: getting participants count for room {room_id} - {result}");
                Ok(result)
            },
            Err(e) => {
                log::error!("rooms_service: failed to get participants count for room {room_id} - \n{e}");
                Err(AppError::InternalServerError(e.to_string()))
            }
        }
    }
}
