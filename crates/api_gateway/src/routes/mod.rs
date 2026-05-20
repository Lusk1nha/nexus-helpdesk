use axum::Router;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::app_state::AppState;

pub mod identity;
pub mod knowledge;
pub mod ticketing;

#[derive(OpenApi)]
#[openapi(
    // Adicionado: Informações gerais da API que aparecerão no cabeçalho do Swagger UI
    info(
        title = "Nexus Helpdesk API",
        version = "1.0.0",
        description = "API RESTful do Nexus Helpdesk.\n\nPlataforma SaaS Multi-Tenant B2B focada em suporte ao cliente com Inteligência Artificial (RAG) nativa.",
        contact(
            name = "Suporte Nexus",
            email = "suporte@nexushelpdesk.local",
            url = "http://localhost:8080"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    // Adicionado: Configuração de servidores (permite testar a API no Swagger em ambientes diferentes)
    servers(
        (url = "http://localhost:8080", description = "Ambiente de Desenvolvimento (Local)"),
        (url = "https://api.nexushelpdesk.com", description = "Ambiente de Produção")
    ),
    paths(
        // Knowledge
        knowledge::handlers::ingest_knowledge_handler,
        knowledge::handlers::list_knowledge_handler,
        knowledge::handlers::search_knowledge_handler,
        knowledge::handlers::delete_knowledge_handler,
        // Ticketing
        ticketing::handlers::create_ticket_handler,
        ticketing::handlers::list_tickets_handler,
        ticketing::handlers::get_ticket_handler,
        ticketing::handlers::update_ticket_status_handler,
        ticketing::handlers::list_ticket_messages_handler,
        ticketing::handlers::add_message_handler,
        ticketing::handlers::approve_ai_response_handler,
        ticketing::handlers::reject_ai_response_handler,
        // Identity
        identity::handlers::register_tenant_handler,
        identity::handlers::login_handler,
        identity::handlers::get_me_handler,
        identity::handlers::admin_reset_user_password_handler,
        identity::handlers::invite_user_handler,
        identity::handlers::list_users_handler,
        identity::handlers::change_user_role_handler,
        identity::handlers::update_user_status_handler,
        identity::handlers::get_tenant_handler,
    ),
    components(
        schemas(
            // Knowledge
            knowledge::contracts::IngestKnowledgePayload,
            knowledge::contracts::IngestKnowledgeResponse,
            knowledge::contracts::KnowledgeDocumentResponse,
            knowledge::contracts::ListKnowledgeResponse,
            knowledge::contracts::SearchKnowledgeResponse,
            knowledge::contracts::SearchResultItem,
            // Ticketing
            ticketing::contracts::CreateTicketPayload,
            ticketing::contracts::CreateTicketResponse,
            ticketing::contracts::TicketResponse,
            ticketing::contracts::UpdateTicketStatusPayload,
            ticketing::contracts::MessageResponse,
            ticketing::contracts::AddMessagePayload,
            // Identity
            identity::contracts::RegisterTenantPayload,
            identity::contracts::RegisterTenantResponse,
            identity::contracts::LoginPayload,
            identity::contracts::LoginResponse,
            identity::contracts::GetMeResponse,
            identity::contracts::AdminResetPasswordPayload,
            identity::contracts::ResetPasswordResponse,
            identity::contracts::InviteUserPayload,
            identity::contracts::InviteUserResponse,
            identity::contracts::TenantMemberResponse,
            identity::contracts::ChangeUserRolePayload,
            identity::contracts::UpdateUserStatusPayload,
            identity::contracts::TenantResponse,
        )
    ),
    tags(
        (name = "Identity", description = "Gerenciamento de Empresas, Usuários e Autenticação"),
        (name = "Ticketing", description = "Gerenciamento de Chamados e Inteligência Artificial"),
        (name = "Knowledge", description = "Gerenciamento de Documentos de Conhecimento")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

pub fn create_router(state: AppState) -> Router {
    let openapi = ApiDoc::openapi();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .nest("/api/v1/identity", identity::routes())
        .nest("/api/v1/tickets", ticketing::routes())
        .nest("/api/v1/knowledge", knowledge::routes())
        .with_state(state)
}
