use actix_web::web;
use crate::domains::auth::handlers::{login, signup, verify_otp};

pub fn configure(cfg: &mut web::ServiceConfig) {   
    cfg.service(
        web::scope("/api/v1/auth")
            .service(signup)
            .service(login)
            .service(verify_otp)
    );
}