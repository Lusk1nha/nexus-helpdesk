use crate::domain::error::DomainError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Agent,
    Customer,
}

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
