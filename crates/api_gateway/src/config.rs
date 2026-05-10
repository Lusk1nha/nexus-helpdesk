#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL deve estar configurada no .env");

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "super_secret_key_apenas_para_desenvolvimento".to_string());

        Self {
            database_url,
            jwt_secret,
        }
    }
}
