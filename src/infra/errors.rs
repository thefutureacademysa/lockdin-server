use crate::domains::auth::error::AppError;

pub fn handle_auth_error(e: AppError) -> actix_web::Result<actix_web::HttpResponse> {
    match e {
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
    }
}