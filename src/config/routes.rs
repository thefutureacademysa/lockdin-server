use crate::domains::auth::handlers::{login, refresh_token, signup, verify_otp};
use crate::domains::rooms::handlers::{get_rooms, join_room, leave_room, participants};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .service(signup)
                    .service(login)
                    .service(verify_otp)
                    .service(refresh_token),
            )
            .service(web::scope("/rooms")
                .service(get_rooms)
                .service(join_room)
                .service(leave_room)
                .service(participants)
            ),
    );
}
