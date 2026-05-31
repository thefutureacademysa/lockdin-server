use crate::config::state::AppState;
use crate::domains::auth::error::AppError;
use crate::infra::middleware::auth::middleware;
use actix_web::web::{Data, Query};
use actix_web::{HttpRequest, HttpResponse, get};
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
        Err(e) => match e {
            AppError::InternalServerError(msg) => {
                log::error!("Middleware auth error: {}", msg);
                Err(actix_web::error::ErrorInternalServerError(
                    serde_json::json!({
                        "error": "InternalServerError",
                        "message": msg
                    })
                    .to_string(),
                ))
            }
            AppError::TokenExpired => {
                log::warn!("Middleware auth error: Token expired");
                Err(actix_web::error::ErrorUnauthorized(
                    serde_json::json!({
                        "error": "TokenExpired",
                        "message": "Access token has expired. Please refresh."
                    })
                    .to_string(),
                ))
            }
            AppError::InvalidToken => {
                log::warn!("Middleware auth error: Invalid token");
                Err(actix_web::error::ErrorUnauthorized(
                    serde_json::json!({
                        "error": "InvalidToken",
                        "message": "Access token is invalid."
                    })
                    .to_string(),
                ))
            }
            AppError::TokenNotFound => {
                log::warn!("Middleware auth error: Token not found");
                Err(actix_web::error::ErrorUnauthorized(serde_json::json!({
                "error": "TokenNotFound",
                "message": "Access token not found."
                    })))
            }
            AppError::AuthorizationHeaderNotFound => {
                log::warn!("Middleware auth error: Authorization header not found");
                Err(actix_web::error::ErrorUnauthorized(serde_json::json!({
                "error": "AuthorizationHeaderNotFound",
                "message": "Authorization header not found."
                    })))
            }
            _ => {
                log::error!("Middleware auth error: {}", e);
                Err(actix_web::error::ErrorInternalServerError(
                    serde_json::json!({
                    "error": "InternalServerError",
                    "message": "An unexpected error occurred."
                        }),
                ))
            }
        },
    }
}
