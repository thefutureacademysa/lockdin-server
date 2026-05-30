use crate::domains::auth::repository::refresh_tokens::postgres_repo::RefreshTokenPostgresRepo;
use crate::domains::auth::repository::verification_codes::postgres_repo::OtpPostgresRepo;
use crate::domains::auth::service::AuthService;
use crate::domains::rooms::postgres::repository::PostgresRoomRepository;
use crate::domains::rooms::service::RoomsService;
use crate::domains::users::implementation::postgres_repo::UserPostgresRepo;
use crate::domains::users::service::UserService;
use crate::infra::database::{init_pool, run_migrations};
use actix_web::web::Data;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Data<AuthService>,
    pub user_service: Data<UserService>,
    pub rooms_service: Data<RoomsService>,
}

pub fn app_state(pg_pool: PgPool) -> AppState {
    let jwt_secret =
        std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable is required");
    AppState {
        auth_service: Data::new(AuthService {
            jwt_secret,
            rt_repo: Arc::new(
                (RefreshTokenPostgresRepo {
                    pool: pg_pool.clone(),
                }),
            ),
            otp_repo: Arc::new(OtpPostgresRepo {
                pool: pg_pool.clone(),
            }),
        }),
        // init user service
        user_service: Data::new(UserService {
            repo: Arc::new(UserPostgresRepo {
                pool: pg_pool.clone(),
            }),
        }),
        rooms_service: Data::new(RoomsService {
            repo: Arc::new(PostgresRoomRepository {
                pool: pg_pool.clone(),
            }),
        }),
    }
}

pub async fn init_state() -> AppState {
    log::info!("Initializing state...");
    let pg_pool = init_pool().await;
    run_migrations(&pg_pool).await;
    let state = app_state(pg_pool);
    state
}
