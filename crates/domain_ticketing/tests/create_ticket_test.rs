mod common;

use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use domain_ticketing::application::use_cases::create_ticket::{
    CreateTicketCommand, CreateTicketUseCase,
};
use domain_ticketing::domain::error::DomainError;

use domain_ticketing::infrastructure::database::postgres_uow::PgTicketingUoWManager;

use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_create_ticket_success_persists_and_queues_to_ai() {
    // 1. Setup da Infraestrutura Efêmera (Banco e Canal na RAM)
    let (pool, _container) = common::setup_isolated_db().await;
    let uow_manager = Arc::new(PgTicketingUoWManager::new(pool.clone()));

    // Criamos um canal com buffer de 10 para capturar o despacho assíncrono do Use Case
    let (ai_sender, mut ai_receiver) = mpsc::channel(10);

    let use_case = CreateTicketUseCase::new(uow_manager.clone(), ai_sender);

    let tenant_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();

    common::seed_test_tenant(&pool, tenant_id).await;
    common::seed_test_user(&pool, customer_id).await;

    let command = CreateTicketCommand {
        tenant_id,
        customer_id,
        title: "Minha internet caiu".to_string(),
        description: "Estou sem sinal desde ontem à noite na minha filial.".to_string(),
    };

    // 2. Executa o Caso de Uso
    let result = use_case.execute(command).await;

    // 3. Asserts do Retorno Síncrono
    // REMOVA OU COMENTE A LINHA ABAIXO:
    // assert!(result.is_ok(), "O caso de uso falhou ao executar");

    let ticket = match result {
        Ok(t) => t,
        Err(e) => panic!("🚀 ERRO DE DOMÍNIO DETECTADO: {:?}", e),
    };

    assert_eq!(ticket.tenant_id, tenant_id);
    assert_eq!(ticket.customer_id, customer_id);
    assert_eq!(ticket.title, "Minha internet caiu");

    // 4. Asserts de Persistência Relacional (Inspeciona o banco via Unit of Work)
    // Trazemos a trait para o escopo para liberar o método .begin()
    use domain_ticketing::domain::ports::TicketingUnitOfWorkManager as _;
    let mut uow = uow_manager.begin().await.unwrap();

    // Valida se o ticket foi gravado na tabela
    let saved_ticket = uow.tickets().find_by_id(ticket.id).await.unwrap();
    assert!(
        saved_ticket.is_some(),
        "O ticket não foi encontrado no banco"
    );

    // Valida se a mensagem humana inicial foi criada e vinculada
    let messages = uow.messages().find_by_ticket_id(ticket.id).await.unwrap();
    assert_eq!(
        messages.len(),
        1,
        "Deveria existir exatamente 1 mensagem vinculada ao ticket"
    );
    assert_eq!(
        messages[0].content,
        "Estou sem sinal desde ontem à noite na minha filial."
    );

    // 5. Assert do Comportamento Assíncrono (Validando o canal MPSC da IA)
    // O Use Case DEVE ter colocado a mensagem na fila antes de retornar Ok
    let received_task = ai_receiver.try_recv();
    assert!(
        received_task.is_ok(),
        "Nenhuma tarefa foi enviada para o canal da IA"
    );

    let task = received_task.unwrap();
    assert_eq!(task.ticket_id, ticket.id);
    assert_eq!(task.tenant_id, tenant_id);
    assert_eq!(
        task.context,
        "Estou sem sinal desde ontem à noite na minha filial."
    );
}

#[tokio::test]
async fn test_create_ticket_fails_if_description_is_empty() {
    let (pool, _container) = common::setup_isolated_db().await;
    let uow_manager = Arc::new(PgTicketingUoWManager::new(pool.clone()));
    let (ai_sender, _ai_receiver) = mpsc::channel(1);

    let use_case = CreateTicketUseCase::new(uow_manager, ai_sender);

    let command = CreateTicketCommand {
        tenant_id: Uuid::new_v4(),
        customer_id: Uuid::new_v4(),
        title: "Problema no app".to_string(),
        description: "   ".to_string(), // Descrição em branco (apenas espaços)
    };

    // 2. Executa e valida o erro de domínio esperado
    let result = use_case.execute(command).await;

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::EmptyMessageContent
    ));
}
