use axum::Router;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::app_state::AppState;

pub mod identity;
pub mod ticketing;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Ticketing
        ticketing::handlers::create_ticket_handler,
        ticketing::handlers::list_tickets_handler,
        ticketing::handlers::get_ticket_handler,
        ticketing::handlers::update_ticket_status_handler,
        ticketing::handlers::list_ticket_messages_handler,
        ticketing::handlers::add_message_handler,
        // Identity
        identity::handlers::register_tenant_handler,
        identity::handlers::login_handler,
        identity::handlers::get_me_handler,
        identity::handlers::admin_reset_user_password_handler,
        identity::handlers::invite_user_handler,
        identity::handlers::list_users_handler,
    ),
    components(
        schemas(
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
        )
    ),
    tags(
        (name = "Identity", description = "Gerenciamento de Empresas, Usuários e Autenticação"),
        (name = "Ticketing", description = "Gerenciamento de Chamados e Inteligência Artificial")
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
        .with_state(state)
}
