// crates/api_gateway/src/app_state.rs

use sqlx::PgPool;
use std::sync::Arc;

use domain_identity::application::use_cases::register_tenant::RegisterTenantUseCase;
use domain_identity::infrastructure::database::postgres_uow::PgUnitOfWorkManager;
use domain_identity::infrastructure::database::postgres_user_repo::PgUserRepository;
use domain_identity::infrastructure::security::argon2_hasher::Argon2Hasher;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub config: Arc<AppConfig>, // 🚀 Mudança de performance: Envolto em Arc!

    // Conforme o app crescer, você pode agrupar os UseCases em um struct 'UseCases'
    // para o AppState não ficar gigante, mas por agora está ótimo assim:
    pub register_tenant_use_case: Arc<RegisterTenantUseCase>,
}

impl AppState {
    pub fn new(db_pool: PgPool, config: AppConfig) -> Self {
        // 1. Instanciamos os Adapters
        let user_repo = Arc::new(PgUserRepository::new(db_pool.clone()));
        let uow_manager = Arc::new(PgUnitOfWorkManager::new(db_pool.clone()));
        let password_hasher = Arc::new(Argon2Hasher::new());

        // 2. Injetamos as dependências no Use Case
        let register_tenant_use_case = Arc::new(RegisterTenantUseCase::new(
            user_repo,
            uow_manager,
            password_hasher,
        ));

        Self {
            db_pool,
            config: Arc::new(config),
            register_tenant_use_case,
        }
    }
}
