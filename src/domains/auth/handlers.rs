use actix_web::{post, web, HttpRequest, HttpResponse};
use crate::config::state::AppState;
use crate::domains::auth::error::AppError;
use crate::domains::auth::models::{LoginRequest, SignupRequest, VerifyOtpRequest};

// signup
// login
// verify-otp
#[post("/signup")]
pub async fn signup(
    state: web::Data<AppState>,
    payload: web::Json<SignupRequest>,
    req: HttpRequest
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
    req: HttpRequest
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
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    match state.auth_service.verify_otp(payload.into_inner()).await {
        Ok(otp) => {
            log::info!("OTP {otp}");
            Ok(HttpResponse::Ok().json(otp))

        }
        Err(e) => {
            log::error!("Error: {}", e);
            Ok(HttpResponse::from_error(actix_web::Error::from(e)))
        }
    }
}