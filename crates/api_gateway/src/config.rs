#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub host: [u8; 4],
    pub frontend_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL deve estar configurada no .env");

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "super_secret_key_apenas_para_desenvolvimento".to_string());

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("PORT deve ser um número válido (ex: 8080)");

        let frontend_url =
            std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

        Self {
            database_url,
            jwt_secret,
            port,
            host: [0, 0, 0, 0],
            frontend_url,
        }
    }
}
