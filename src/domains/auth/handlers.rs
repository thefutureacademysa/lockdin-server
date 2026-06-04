use actix_web::{post, web, HttpResponse};
use crate::config::state::AppState;
use crate::domains::auth::error::AppError;
use crate::domains::auth::models::{LoginRequest, RefreshRequest, RefreshResponse, SignupRequest, VerifyOtpRequest};
use crate::domains::auth::service::AuthService;

// signup
// login
// verify-otp
#[post("/signup")]
pub async fn signup(
    state: web::Data<AppState>,
    payload: web::Json<SignupRequest>,
) -> actix_web::Result<HttpResponse> {
    let user_service = state.user_service.clone();
    match state.auth_service.signup(user_service, payload.into_inner()).await {
        Ok(otp) => {
            log::info!("OTP Issued: {}", otp.code);
            Ok(HttpResponse::Ok().json(otp))

        }
        Err(e) => {
            log::error!("Error: {}", e);
            Ok(HttpResponse::from_error(actix_web::Error::from(e)))
        }
    }
}

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    payload: web::Json<LoginRequest>,
) -> actix_web::Result<HttpResponse> {
    let user_service = state.user_service.clone();
    match state.auth_service.login(user_service, payload.into_inner()).await {
        Ok(otp) => {
            log::info!("OTP Issued: {}", otp.code);
            Ok(HttpResponse::Ok().json(otp))

        }
        Err(e) => {
            log::error!("Error: {}", e);
            Ok(HttpResponse::from_error(actix_web::Error::from(e)))
        }
    }
}

#[post("/verify-otp")]
pub async fn verify_otp(
    state: web::Data<AppState>,
    payload: web::Json<VerifyOtpRequest>,
) -> actix_web::Result<HttpResponse> {
    let user_service = state.user_service.clone();
    match state.auth_service.verify_otp(user_service, payload.into_inner()).await {
        Ok(tokens) => {
            log::info!("OTP verified successfully. Access token and refresh token issued.");
            Ok(HttpResponse::Ok().json(tokens))
        }
        Err(e) => {
            log::error!("Error: {}", e);
            Ok(HttpResponse::from_error(actix_web::Error::from(e)))
        }
    }
}

#[post("/refresh")]
pub async fn refresh_token(
    state: web::Data<AppState>,
    payload: web::Json<RefreshRequest>,
) -> actix_web::Result<HttpResponse> {
    match state.auth_service
        .refresh_access_token(&payload.refresh_token)
        .await {
        Ok(access_token) => {
            log::info!("POST /auth/refresh  - access token refreshed successfully.");
            Ok(HttpResponse::Ok().json(access_token))
        },
        Err(e) => {
            log::error!("POST /auth/refresh - failed refreshing access token. \n{}", e);
            Ok(HttpResponse::from_error(actix_web::Error::from(e)))
        }
    }
}