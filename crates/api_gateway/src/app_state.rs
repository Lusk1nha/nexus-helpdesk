use sqlx::PgPool;
use std::sync::Arc;

// Importe as traits e o Use Case do seu domínio
use domain_identity::application::use_cases::register_tenant::RegisterTenantUseCase;

// Importe as implementações de infraestrutura (ajuste os paths conforme seu projeto)
use domain_identity::infrastructure::database::postgres_uow::PgUnitOfWorkManager;
use domain_identity::infrastructure::database::postgres_user_repo::PgUserRepository;
use domain_identity::infrastructure::security::argon2_hasher::Argon2Hasher;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub config: AppConfig,
    pub register_tenant_use_case: Arc<RegisterTenantUseCase>,
}

impl AppState {
    pub fn new(db_pool: PgPool, config: AppConfig) -> Self {
        // 1. Correção aqui: Instanciar PgUserRepository para o user_repo
        let user_repo = Arc::new(PgUserRepository::new(db_pool.clone()));

        // 2. Instanciar PgUnitOfWorkManager para o uow_manager
        let uow_manager = Arc::new(PgUnitOfWorkManager::new(db_pool.clone()));

        // 3. Instanciar o Hasher
        let password_hasher = Arc::new(Argon2Hasher::new());

        // 4. Injetar as dependências no Use Case
        let register_tenant_use_case = Arc::new(RegisterTenantUseCase::new(
            user_repo, // Agora o Rust aceita, pois é um PgUserRepository!
            uow_manager,
            password_hasher,
        ));

        Self {
            db_pool,
            config,
            register_tenant_use_case,
        }
    }
}
