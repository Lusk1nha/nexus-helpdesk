use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    // --- Erros de Regra de Negócio (Business Logic) ---
    
    #[error("Este endereço de e-mail já está em uso.")]
    UserAlreadyExists,

    #[error("Usuário não encontrado.")]
    UserNotFound,

    #[error("Credenciais inválidas. Verifique seu e-mail e senha.")]
    InvalidCredentials,

    #[error("Tenant (Empresa) não encontrado.")]
    TenantNotFound,

    #[error("A função (Role) '{0}' é inválida. Use: admin, agent, customer.")]
    InvalidRole(String),

    // --- Erros de Infraestrutura (Encapsulados) ---
    // Note que usamos String para não acoplar o domínio aos erros do SQLx ou Argon2
    
    #[error("Erro interno no banco de dados: {0}")]
    DatabaseError(String),

    #[error("Erro interno ao processar segurança/criptografia: {0}")]
    SecurityError(String),
}