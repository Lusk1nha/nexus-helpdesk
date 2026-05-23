use ai_engine::AiEngine;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use domain_ticketing::application::{
    AddMessageToTicketUseCase, AiTask, CreateTicketUseCase, GetTicketUseCase,
    ListTicketMessagesUseCase, ListTicketsUseCase, UpdateTicketStatusUseCase,
};
use domain_ticketing::infrastructure::database::postgres_uow::PgTicketingUoWManager;

use domain_identity::application::use_cases::{
    AuthenticateApiKeyUseCase, ChangeUserRoleUseCase, CheckSlugAvailabilityUseCase,
    CreateApiKeyUseCase, GetTenantUseCase, InviteUserUseCase, IssueRefreshTokenUseCase,
    ListApiKeysUseCase, ListUsersUseCase, LoginUseCase, LogoutUseCase, RefreshSessionUseCase,
    RegisterTenantUseCase, ResetPasswordUseCase, RevokeApiKeyUseCase, UpdateUserStatusUseCase,
};
use domain_identity::infrastructure::{
    database::postgres_uow::PgUnitOfWorkManager, security::argon2_hasher::Argon2Hasher,
};

use crate::config::AppConfig;
use crate::realtime::RealtimeHub;

pub struct IdentityUseCases {
    pub register_tenant: Arc<RegisterTenantUseCase>,
    pub check_slug: Arc<CheckSlugAvailabilityUseCase>,
    pub login: Arc<LoginUseCase>,
    pub reset_password: Arc<ResetPasswordUseCase>,
    pub invite_user: Arc<InviteUserUseCase>,
    pub list_users: Arc<ListUsersUseCase>,
    pub change_user_role: Arc<ChangeUserRoleUseCase>,
    pub update_user_status: Arc<UpdateUserStatusUseCase>,
    pub get_tenant: Arc<GetTenantUseCase>,
    pub issue_refresh_token: Arc<IssueRefreshTokenUseCase>,
    pub refresh_session: Arc<RefreshSessionUseCase>,
    pub logout: Arc<LogoutUseCase>,
    pub create_api_key: Arc<CreateApiKeyUseCase>,
    pub revoke_api_key: Arc<RevokeApiKeyUseCase>,
    pub list_api_keys: Arc<ListApiKeysUseCase>,
    pub authenticate_api_key: Arc<AuthenticateApiKeyUseCase>,
}

pub struct TicketingUseCases {
    pub create_ticket: Arc<CreateTicketUseCase>,
    pub get_ticket: Arc<GetTicketUseCase>,
    pub list_tickets: Arc<ListTicketsUseCase>,
    pub update_ticket_status: Arc<UpdateTicketStatusUseCase>,
    pub add_message: Arc<AddMessageToTicketUseCase>,
    pub list_ticket_messages: Arc<ListTicketMessagesUseCase>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub identity: Arc<IdentityUseCases>,
    pub ticketing: Arc<TicketingUseCases>,
    pub ai_engine: Arc<AiEngine>,
    pub realtime: Arc<RealtimeHub>,
}

impl AppState {
    pub fn new(
        db_pool: PgPool,
        config: AppConfig,
        ai_queue_sender: Sender<AiTask>,
        ai_engine: Arc<AiEngine>,
        realtime: Arc<RealtimeHub>,
    ) -> Self {
        let identity_uow_manager = Arc::new(PgUnitOfWorkManager::new(db_pool.clone()));
        let ticketing_uow_manager = Arc::new(PgTicketingUoWManager::new(db_pool));
        let password_hasher = Arc::new(Argon2Hasher::new());

        // Identity use cases
        let register_tenant = Arc::new(RegisterTenantUseCase::new(
            identity_uow_manager.clone(),
            password_hasher.clone(),
        ));
        let check_slug = Arc::new(CheckSlugAvailabilityUseCase::new(
            identity_uow_manager.clone(),
        ));
        let login = Arc::new(LoginUseCase::new(
            identity_uow_manager.clone(),
            password_hasher.clone(),
        ));
        let reset_password = Arc::new(ResetPasswordUseCase::new(
            identity_uow_manager.clone(),
            password_hasher.clone(),
        ));
        let invite_user = Arc::new(InviteUserUseCase::new(
            identity_uow_manager.clone(),
            password_hasher,
        ));
        let list_users = Arc::new(ListUsersUseCase::new(identity_uow_manager.clone()));
        let change_user_role = Arc::new(ChangeUserRoleUseCase::new(identity_uow_manager.clone()));
        let update_user_status =
            Arc::new(UpdateUserStatusUseCase::new(identity_uow_manager.clone()));
        let get_tenant = Arc::new(GetTenantUseCase::new(identity_uow_manager.clone()));
        let issue_refresh_token =
            Arc::new(IssueRefreshTokenUseCase::new(identity_uow_manager.clone()));
        let refresh_session = Arc::new(RefreshSessionUseCase::new(identity_uow_manager.clone()));
        let logout = Arc::new(LogoutUseCase::new(identity_uow_manager.clone()));
        let create_api_key = Arc::new(CreateApiKeyUseCase::new(identity_uow_manager.clone()));
        let revoke_api_key = Arc::new(RevokeApiKeyUseCase::new(identity_uow_manager.clone()));
        let list_api_keys = Arc::new(ListApiKeysUseCase::new(identity_uow_manager.clone()));
        let authenticate_api_key = Arc::new(AuthenticateApiKeyUseCase::new(identity_uow_manager));

        let identity_cases = Arc::new(IdentityUseCases {
            register_tenant,
            check_slug,
            login,
            reset_password,
            invite_user,
            list_users,
            change_user_role,
            update_user_status,
            get_tenant,
            issue_refresh_token,
            refresh_session,
            logout,
            create_api_key,
            revoke_api_key,
            list_api_keys,
            authenticate_api_key,
        });

        // Ticketing use cases
        let create_ticket = Arc::new(CreateTicketUseCase::new(
            ticketing_uow_manager.clone(),
            ai_queue_sender,
        ));
        let get_ticket = Arc::new(GetTicketUseCase::new(ticketing_uow_manager.clone()));
        let list_tickets = Arc::new(ListTicketsUseCase::new(ticketing_uow_manager.clone()));
        let update_ticket_status = Arc::new(UpdateTicketStatusUseCase::new(
            ticketing_uow_manager.clone(),
        ));
        let add_message = Arc::new(AddMessageToTicketUseCase::new(
            ticketing_uow_manager.clone(),
        ));
        let list_ticket_messages = Arc::new(ListTicketMessagesUseCase::new(ticketing_uow_manager));

        let ticketing_cases = Arc::new(TicketingUseCases {
            create_ticket,
            get_ticket,
            list_tickets,
            update_ticket_status,
            add_message,
            list_ticket_messages,
        });

        Self {
            config: Arc::new(config),
            identity: identity_cases,
            ticketing: ticketing_cases,
            ai_engine,
            realtime,
        }
    }
}
