#[derive(Clone)]
pub struct EnvironmentVars {
    pub port: u16,
    pub host: String,
    pub frontend_origin: String,
}

impl EnvironmentVars {
    pub fn init() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .unwrap();
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let frontend_origin =
            std::env::var("FRONTEND_ORIGIN").unwrap_or("http://localhost:3000".to_string());
        Self {
            port,
            host,
            frontend_origin,
        }
    }
}
