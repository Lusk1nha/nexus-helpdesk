use domain_identity::application::use_cases::ResetPasswordUseCase;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

// --- Imports de Ticketing ---
use domain_ticketing::application::{AiTask, CreateTicketUseCase};
use domain_ticketing::infrastructure::database::postgres_uow::PgTicketingUoWManager;

// --- Imports de Identity ---
use domain_identity::application::use_cases::{
    login::LoginUseCase, register_tenant::RegisterTenantUseCase,
};
use domain_identity::infrastructure::{
    database::postgres_uow::PgUnitOfWorkManager, security::argon2_hasher::Argon2Hasher,
};

use crate::config::AppConfig;

// ==========================================
// 📦 Agrupamentos Estratégicos (Clean State)
// ==========================================

pub struct IdentityUseCases {
    pub register_tenant: Arc<RegisterTenantUseCase>,
    pub login: Arc<LoginUseCase>,
    pub reset_password: Arc<ResetPasswordUseCase>,
}

pub struct TicketingUseCases {
    pub create_ticket: Arc<CreateTicketUseCase>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub identity: Arc<IdentityUseCases>,
    pub ticketing: Arc<TicketingUseCases>,
}

impl AppState {
    pub fn new(db_pool: PgPool, config: AppConfig, ai_queue_sender: Sender<AiTask>) -> Self {
        // ---------------------------------------------------------
        // 1. Setup da Infraestrutura Compartilhada
        // ---------------------------------------------------------
        let identity_uow_manager = Arc::new(PgUnitOfWorkManager::new(db_pool.clone()));
        let ticketing_uow_manager = Arc::new(PgTicketingUoWManager::new(db_pool));

        let password_hasher = Arc::new(Argon2Hasher::new());

        // ---------------------------------------------------------
        // 2. Setup dos Use Cases de Identity
        // ---------------------------------------------------------

        let register_tenant = Arc::new(RegisterTenantUseCase::new(
            identity_uow_manager.clone(),
            password_hasher.clone(),
        ));

        let login = Arc::new(LoginUseCase::new(
            identity_uow_manager.clone(),
            password_hasher.clone(),
        ));

        let reset_password = Arc::new(ResetPasswordUseCase::new(
            identity_uow_manager,
            password_hasher,
        ));

        let identity_cases = Arc::new(IdentityUseCases {
            register_tenant,
            login,
            reset_password,
        });

        // ---------------------------------------------------------
        // 3. Setup dos Use Cases de Ticketing
        // ---------------------------------------------------------
        let create_ticket = Arc::new(CreateTicketUseCase::new(
            ticketing_uow_manager,
            ai_queue_sender,
        ));

        let ticketing_cases = Arc::new(TicketingUseCases { create_ticket });

        // ---------------------------------------------------------
        // 4. Montagem do Estado Final
        // ---------------------------------------------------------
        Self {
            config: Arc::new(config),

            identity: identity_cases,
            ticketing: ticketing_cases,
        }
    }
}
