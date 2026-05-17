pub mod config;
pub mod infra;
pub mod domains;

use env_logger::Env;
use crate::config::server::run;
use crate::config::state::init_state;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv::dotenv().ok();
    
    let state = init_state().await;

    run(state).await
}