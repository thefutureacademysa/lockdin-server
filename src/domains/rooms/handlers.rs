use crate::config::state::AppState;
use actix_web::web::{Data, Path, Query};
use actix_web::{HttpResponse, get};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoomsFilter {
    pub grade: Option<i32>,
    pub category: Option<String>,
}

#[get("")]
pub async fn get_all_rooms(
    state: Data<AppState>,
    filters: Query<RoomsFilter>,
) -> actix_web::Result<HttpResponse> {

    let rf = filters.into_inner();
    let opt_grade = rf.grade.clone();
    let opt_category = rf.category.clone();

    match state
        .rooms_service
        .get_all_rooms(opt_grade, opt_category)
        .await
    {
        Ok(rooms) => {
            log::info!("Rooms fetched successfully");
            Ok(HttpResponse::Ok().json(rooms))
        }
        Err(e) => {
            log::error!("Error: {}", e);
            Ok(HttpResponse::from_error(actix_web::Error::from(e)))
        }
    }
}
