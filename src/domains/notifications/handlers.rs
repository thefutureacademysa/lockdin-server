use actix_web::{post, HttpResponse};

#[post("/notifications/send-otp")]
pub async fn send_otp() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}