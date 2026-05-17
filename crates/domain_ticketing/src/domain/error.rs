use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    // --- Erros de Entidade / Buscas ---
    #[error("Ticket não encontrado.")]
    TicketNotFound,

    #[error("Mensagem não encontrada.")]
    MessageNotFound,

    // --- Erros de Regra de Negócio & Máquina de Estados ---
    #[error("Acesso negado: Este ticket pertence a outra empresa (Tenant).")]
    UnauthorizedTenantAccess,

    #[error("Operação inválida: O ticket '{0}' já está encerrado e não pode ser modificado.")]
    TicketAlreadyClosed(Uuid),

    #[error("Transição de status inválida. Não é possível mudar de '{from}' para '{to}'.")]
    InvalidStatusTransition { from: String, to: String },

    #[error("O conteúdo da mensagem não pode estar vazio.")]
    EmptyMessageContent,

    // --- Erros de Integração Assíncrona (AI Worker) ---
    #[error("O motor de IA falhou ao processar o ticket: {0}")]
    AiEngineError(String),

    // --- Erros de Infraestrutura (Encapsulados) ---
    #[error("Erro interno no banco de dados: {0}")]
    DatabaseError(String),
}
