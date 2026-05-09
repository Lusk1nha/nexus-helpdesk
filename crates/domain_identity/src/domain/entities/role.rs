use crate::domain::error::DomainError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Admin,
    Agent,
    Customer,
}

// Convertendo para String (Útil para salvar no PostgreSQL ou enviar no JWT)
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            Role::Admin => "admin",
            Role::Agent => "agent",
            Role::Customer => "customer",
        };
        write!(f, "{}", role_str)
    }
}

// Lendo do Banco de Dados ou da API de forma segura
impl FromStr for Role {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "agent" => Ok(Role::Agent),
            "customer" => Ok(Role::Customer),
            _ => Err(DomainError::InvalidRole(s.to_string())),
        }
    }
}
