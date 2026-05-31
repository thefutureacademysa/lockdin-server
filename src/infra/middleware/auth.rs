use crate::domains::auth::error::AppError;
use crate::domains::auth::models::Claims;
use actix_web::HttpRequest;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

pub async fn middleware(req: HttpRequest) -> actix_web::Result<Claims, AppError> {
    match req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        Some(auth_val) => match auth_val.strip_prefix("Bearer ") {
            Some(jsonwebtoken) => match verify_token(jsonwebtoken.trim()) {
                Ok(claims) => Ok(claims),
                Err(e) => Err(e)
            },
            None => {
                log::error!("No token found in request headers");
                Err(AppError::TokenNotFound)
            }
        },
        None => {
            log::error!("Authorization header not found in request headers:");
            Err(AppError::AuthorizationHeaderNotFound)
        }
    }
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    match std::env::var("JWT_SECRET") {
        Ok(secret) => {
            let decoding_key = DecodingKey::from_secret(secret.as_bytes());
            let validation = Validation::new(Algorithm::HS256);

            match decode::<Claims>(token, &decoding_key, &validation) {
                Ok(token_data) => Ok(token_data.claims),
                Err(e) => match e.kind() {
                    ErrorKind::ExpiredSignature => Err(AppError::TokenExpired),
                    _ => Err(AppError::InvalidToken),
                },
            }
        }
        Err(e) => {
            log::error!("Verify token error: {}", e);
            Err(AppError::InternalServerError(e.to_string()))
        }
    }
}
