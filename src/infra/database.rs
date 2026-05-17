use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn init_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create PgPool");

    pg_pool
}

pub async fn run_migrations(pool: &PgPool) {
    log::info!("Running migrations");
    let _ = sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| log::error!("{}", e.to_string()));
}