use actix_cors::Cors;
use actix_web::http::header;
use crate::infra::env_vars::EnvironmentVars;

pub fn build_cors() -> Cors {
    let ev = EnvironmentVars::init();
    Cors::default()
        .allowed_origin(&ev.frontend_origin)
        .allowed_methods(["GET", "POST", "DELETE"])
        .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .supports_credentials()
        .max_age(3600)
}