use crate::domains::auth::handlers::{login, signup, verify_otp};
use crate::domains::rooms::handlers::{get_rooms};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .service(signup)
                    .service(login)
                    .service(verify_otp),
            )
            .service(web::scope("/rooms").service(get_rooms)),
    );
}
