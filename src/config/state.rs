use crate::domains::auth::service::AuthService;
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
}

pub fn app_state(pg_pool: PgPool) -> AppState {
    AppState {
        auth_service: Data::new(AuthService {}),
        // init user service
        user_service: Data::new(UserService {
            repo: Arc::new(UserPostgresRepo {
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
