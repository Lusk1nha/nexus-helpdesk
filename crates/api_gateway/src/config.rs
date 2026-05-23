use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    // ── Database ─────────────────────────────────────────────────────────────
    /// PostgreSQL connection string. Required — no default.
    /// Example: postgres://user:pass@localhost:5432/nexus_helpdesk
    pub database_url: String,

    // ── Security ─────────────────────────────────────────────────────────────
    /// Secret used to sign and verify JWTs. Required — no default.
    /// Use a long random string in production (32+ characters).
    pub jwt_secret: String,

    /// Lifetime of access tokens, in minutes. Default: 15.
    pub access_token_ttl_minutes: u32,

    /// Lifetime of refresh tokens, in days. Default: 30.
    pub refresh_token_ttl_days: u32,

    /// Issuer claim (`iss`) put into every JWT. Default: "nexus-helpdesk".
    pub jwt_issuer: String,

    // ── HTTP server ───────────────────────────────────────────────────────────
    /// IP address the server binds to. Default: 0.0.0.0
    pub host: String,

    /// Port the server listens on. Default: 8080
    pub port: u16,

    /// Allowed CORS origin for the frontend. Default: http://localhost:5173
    pub frontend_url: String,

    // ── AI integrations ───────────────────────────────────────────────────────
    /// Base URL for the local Ollama LLM server. Default: http://127.0.0.1:11434
    pub ollama_url: String,

    /// Base URL for the Qdrant vector database. Default: http://127.0.0.1:6334
    pub qdrant_url: String,
}

impl AppConfig {
    /// Loads configuration from three layered sources (last wins):
    ///
    /// 1. **Defaults** — hard-coded safe values for optional settings.
    /// 2. **`config.toml`** — optional local file, useful for per-machine overrides.
    ///    Follows the same snake_case field names as the struct.
    /// 3. **Environment variables** — override everything.
    ///    `DATABASE_URL` → `database_url`, `JWT_SECRET` → `jwt_secret`, etc.
    ///    (Call `dotenvy::dotenv()` before this to load a `.env` file first.)
    ///
    /// # Panics
    /// Panics at startup when a **required** variable (`DATABASE_URL`, `JWT_SECRET`)
    /// is missing or any value cannot be parsed into its expected type.
    pub fn load() -> Self {
        Self::try_load().unwrap_or_else(|e| {
            panic!(
                "❌ Falha ao carregar configuração da aplicação:\n\n  {e}\n\n\
                Verifique se todas as variáveis obrigatórias estão no .env ou no ambiente.\n\
                Variáveis obrigatórias: DATABASE_URL, JWT_SECRET"
            )
        })
    }

    fn try_load() -> Result<Self, config::ConfigError> {
        Config::builder()
            // ── Defaults ───────────────────────────────────────────────────
            .set_default("host", "0.0.0.0")?
            .set_default("port", 8080_i64)?
            .set_default("frontend_url", "http://localhost:5173")?
            .set_default("ollama_url", "http://127.0.0.1:11434")?
            .set_default("qdrant_url", "http://127.0.0.1:6334")?
            .set_default("access_token_ttl_minutes", 15_i64)?
            .set_default("refresh_token_ttl_days", 30_i64)?
            .set_default("jwt_issuer", "nexus-helpdesk")?
            // ── Optional config file ───────────────────────────────────────
            // Place a config.toml at the workspace root to override defaults
            // without touching environment variables.
            .add_source(File::with_name("config").required(false))
            // ── Environment variables ──────────────────────────────────────
            // Env vars are read case-insensitively: DATABASE_URL → database_url
            .add_source(Environment::default())
            .build()?
            .try_deserialize::<AppConfig>()
            .and_then(|cfg| {
                if cfg.jwt_secret.len() < 32 {
                    return Err(config::ConfigError::Message(
                        "JWT_SECRET deve ter pelo menos 32 caracteres (gere com `openssl rand -base64 48`)."
                            .to_string(),
                    ));
                }
                Ok(cfg)
            })
    }
}
