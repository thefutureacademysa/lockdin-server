use crate::config::state::AppState;
use crate::infra::errors::handle_auth_error;
use crate::infra::middleware::auth::middleware;
use actix_web::web::{Data, Query};
use actix_web::{HttpRequest, HttpResponse, get, post, web};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoomQueryParams {
    pub search: Option<String>,
    pub grade: Option<i32>,
    pub category: Option<String>,
}

#[get("")]
pub async fn get_rooms(
    state: Data<AppState>,
    query: Query<RoomQueryParams>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} requested rooms", claims.sub);

            let rqp = query.into_inner();
            let opt_search = rqp.search.clone();
            let opt_grade = rqp.grade.clone();
            let opt_category = rqp.category.clone();

            match state
                .rooms_service
                .get_rooms(opt_search, opt_grade, opt_category)
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
        Err(e) => handle_auth_error(e),
    }
}

#[post("/{room_id}/join")]
pub async fn join_room(
    state: Data<AppState>,
    req: HttpRequest,
    room_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} requested to join room {}", claims.sub, room_id);
            match state
                .rooms_service
                .join_room(room_id.into_inner().as_str(), claims.sub.as_str())
                .await
            {
                Ok(result) => Ok(HttpResponse::Ok().json(result)),
                Err(e) => {
                    log::error!("Error: {}", e);
                    Ok(HttpResponse::from_error(actix_web::Error::from(e)))
                }
            }
        }
        Err(e) => handle_auth_error(e),
    }
}

#[post("/{room_id}/leave")]
pub async fn leave_room(
    state: Data<AppState>,
    req: HttpRequest,
    room_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} requested to leave room {}", claims.sub, room_id);
            match state
                .rooms_service
                .leave_room(room_id.into_inner().as_str(), claims.sub.as_str())
                .await
            {
                Ok(result) => Ok(HttpResponse::Ok().json(result)),
                Err(e) => {
                    log::error!("Error: {}", e);
                    Ok(HttpResponse::from_error(actix_web::Error::from(e)))
                }
            }
        }
        Err(e) => handle_auth_error(e),
    }
}

#[get("/{room_id}/participants")]
pub async fn participants(
    state: Data<AppState>,
    req: HttpRequest,
    room_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            match state
                .rooms_service
                .participants(room_id.into_inner().as_str())
                .await
            {
                Ok(result) => Ok(HttpResponse::Ok().json(result)),
                Err(e) => {
                    log::error!("POST /rooms/participants - Internal Server Error");
                    Ok(HttpResponse::from_error(actix_web::Error::from(e)))
                }
            }
        }
        Err(e) => handle_auth_error(e),
    }
}
