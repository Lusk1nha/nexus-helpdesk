mod common;

use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use ai_engine::AiEngine;
use domain_ticketing::application::use_cases::create_ticket::{
    CreateTicketCommand, CreateTicketUseCase,
};
use domain_ticketing::application::workers::ai_worker::AiWorker;
use domain_ticketing::domain::entities::ticket::TicketStatus;
use domain_ticketing::domain::ports::TicketingUnitOfWorkManager as _;
use domain_ticketing::infrastructure::database::postgres_uow::PgTicketingUoWManager;

fn dummy_ai_engine(ollama_url: &str) -> Arc<AiEngine> {
    Arc::new(
        AiEngine::new(
            reqwest::Client::new(),
            ollama_url.to_string(),
            "http://127.0.0.1:1",
        )
        .expect("Failed to build test AiEngine"),
    )
}

#[tokio::test]
async fn test_worker_reverts_ticket_to_open_when_ollama_is_unavailable() {
    let (pool, _container) = common::setup_isolated_db().await;

    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    common::seed_test_tenant(&pool, tenant_id).await;
    common::seed_test_user(&pool, user_id).await;

    let uow_manager = Arc::new(PgTicketingUoWManager::new(pool.clone()));

    // Use a bad Ollama URL — connection will be refused immediately.
    let bad_ollama_url = "http://127.0.0.1:1".to_string();

    let (ai_sender, ai_receiver) = mpsc::channel(10);
    let create_uc = CreateTicketUseCase::new(uow_manager.clone(), ai_sender);

    let ticket = create_uc
        .execute(CreateTicketCommand {
            tenant_id,
            customer_id: user_id,
            title: "Printer on fire".to_string(),
            description: "My printer is literally on fire, please help.".to_string(),
        })
        .await
        .expect("ticket creation should succeed");

    // Dropping the use case closes its copy of the sender — once the worker
    // drains the one buffered task the channel will be empty and start() exits.
    drop(create_uc);

    let worker = AiWorker::new(
        ai_receiver,
        uow_manager.clone(),
        bad_ollama_url.clone(),
        dummy_ai_engine(&bad_ollama_url),
    );
    let handle = tokio::spawn(async move { worker.start().await });

    // Worker processes the task (Ollama call fails fast on refused connection)
    // then exits because the sender was dropped.
    handle.await.expect("worker task should not panic");

    // The ticket must be back to Open — not stuck at ProcessingAI.
    let mut uow = uow_manager.begin().await.unwrap();
    let saved = uow
        .tickets()
        .find_by_id(ticket.id)
        .await
        .unwrap()
        .expect("ticket should still exist");

    assert_eq!(
        saved.status,
        TicketStatus::Open,
        "ticket should be reverted to Open after AI failure"
    );
}

/// When Ollama returns a valid response the worker must:
/// 1. Save an AI message in the ticket thread.
/// 2. Transition the ticket to AwaitingAgentApproval.
///
/// This test is skipped in CI because it requires a running Ollama instance.
/// Run manually with: OLLAMA_TEST=1 cargo test --test ai_worker_test test_worker_saves_response
#[tokio::test]
async fn test_worker_saves_ai_response_and_transitions_to_awaiting_approval() {
    if std::env::var("OLLAMA_TEST").is_err() {
        return; // skipped unless explicitly opted in
    }

    let ollama_url =
        std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".to_string());

    let (pool, _container) = common::setup_isolated_db().await;

    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    common::seed_test_tenant(&pool, tenant_id).await;
    common::seed_test_user(&pool, user_id).await;

    let uow_manager = Arc::new(PgTicketingUoWManager::new(pool.clone()));
    let (ai_sender, ai_receiver) = mpsc::channel(10);
    let create_uc = CreateTicketUseCase::new(uow_manager.clone(), ai_sender);

    let ticket = create_uc
        .execute(CreateTicketCommand {
            tenant_id,
            customer_id: user_id,
            title: "Slow internet".to_string(),
            description: "My internet speed dropped to 1 Mbps after the last update.".to_string(),
        })
        .await
        .expect("ticket creation should succeed");

    drop(create_uc);

    let worker = AiWorker::new(
        ai_receiver,
        uow_manager.clone(),
        ollama_url.clone(),
        dummy_ai_engine(&ollama_url),
    );
    tokio::spawn(async move { worker.start().await })
        .await
        .expect("worker should not panic");

    let mut uow = uow_manager.begin().await.unwrap();

    let saved = uow
        .tickets()
        .find_by_id(ticket.id)
        .await
        .unwrap()
        .expect("ticket should exist");

    assert_eq!(saved.status, TicketStatus::AwaitingAgentApproval);

    let messages = uow.messages().find_by_ticket_id(ticket.id).await.unwrap();

    // 1 customer message + 1 AI response
    assert_eq!(messages.len(), 2, "should have customer message + AI reply");

    use domain_ticketing::domain::entities::message::SenderType;
    assert_eq!(messages[1].sender_type, SenderType::AI);
    assert!(!messages[1].content.is_empty());
}
