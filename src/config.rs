#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub environment: String,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/app".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "development_secret".to_string()),
        }
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}
