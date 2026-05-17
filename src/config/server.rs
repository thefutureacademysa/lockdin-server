use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use tokio::io;
use crate::config::cors::build_cors;
use crate::config::routes;
use crate::config::state::AppState;
use crate::infra::env_vars::EnvironmentVars;

pub async fn run(state: AppState) -> io::Result<()> {
    log::info!("running server...");
    let ev = EnvironmentVars::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
            .wrap(build_cors())
            .app_data(Data::new(state.clone()))
            .configure(routes::configure)
    })
        .workers(8)
        .bind((ev.host, ev.port))?
        .run()
        .await
}