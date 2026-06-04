use redis::RedisResult;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/**
* Initializes a Redis connection
* Initializes a Postgres connection pool
* Runs migrations
**/

pub async fn init_redis() -> RedisResult<redis::aio::MultiplexedConnection> {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    log::info!("Connecting to Redis at {redis_url}");
    match redis::Client::open(redis_url) {
        Ok(client) => {
            match client.get_multiplexed_async_connection().await {
                Ok(conn) => Ok(conn),
                Err(e) => {
                    log::error!("Failed to connect to Redis: {e}");
                    Err(e)
                }
            }
        },
        Err(e) => {
            log::info!("Failed to connect to Redis: {e}");
            Err(e)
        }
    }
}

pub async fn init_postgres() -> PgPool {
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
