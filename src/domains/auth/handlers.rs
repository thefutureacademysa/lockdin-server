use actix_web::{post, web, HttpRequest, HttpResponse};
use crate::config::state::AppState;

// signup
// login
// verify-otp
#[post("/signup")]
pub async fn signup(
    state: web::Data<AppState>,
    // req: web::Json<SignupRequest>,
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    // req: web::Json<LoginRequest>,
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[post("/verify-otp")]
pub async fn verify_otp(
    state: web::Data<AppState>,
    // req: web::Json<VerifyOtpRequest>,
    req: HttpRequest
) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}