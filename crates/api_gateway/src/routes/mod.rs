

use axum::Router;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::app_state::AppState;

pub mod identity;
pub mod ticketing;

// 1. Definição da Documentação OpenAPI
#[derive(OpenApi)]
#[openapi(
    paths(
        // Rotas do Ticketing
        ticketing::handlers::create_ticket_handler,
        
        // 🚀 Novas Rotas de Identity adicionadas aqui:
        identity::handlers::register_tenant_handler,
        identity::handlers::login_handler,
        identity::handlers::get_me_handler,
        identity::handlers::admin_reset_user_password_handler
    ),
    components(
        schemas(
            // Schemas do Ticketing
            ticketing::contracts::CreateTicketPayload,
            ticketing::contracts::CreateTicketResponse,
            
            // 🚀 Novos Schemas de Identity adicionados aqui:
            identity::contracts::RegisterTenantPayload,
            identity::contracts::RegisterTenantResponse,
            identity::contracts::GetMeResponse,

            identity::contracts::LoginPayload,
            identity::contracts::LoginResponse,
            
            identity::contracts::AdminResetPasswordPayload,
            identity::contracts::ResetPasswordResponse,
        )
    ),
    tags(
        (name = "Identity", description = "Gerenciamento de Empresas, Usuários e Autenticação"),
        (name = "Ticketing", description = "Gerenciamento de Chamados e Inteligência Artificial")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

// Adiciona o cadeado de Autenticação JWT no Swagger UI
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
    // Gera o JSON do OpenAPI em tempo de compilação
    let openapi = ApiDoc::openapi();

    Router::new()
        // Serve a interface gráfica do Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        // Suas rotas normais
        .nest("/api/v1/identity", identity::routes())
        .nest("/api/v1/tickets", ticketing::routes())
        .with_state(state)
}
