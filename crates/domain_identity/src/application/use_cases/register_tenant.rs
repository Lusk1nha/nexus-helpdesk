use std::sync::Arc;

use crate::domain::entities::{Credential, Role, Tenant, TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::{PasswordHasher, UnitOfWorkManager, UserRepository};

pub struct RegisterTenantCommand {
    pub tenant_name: String,
    pub admin_full_name: String,
    pub admin_email: String,
    pub admin_plain_password: String,
}

pub struct RegisterTenantUseCase {
    user_repo: Arc<dyn UserRepository>,
    uow_manager: Arc<dyn UnitOfWorkManager>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl RegisterTenantUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        uow_manager: Arc<dyn UnitOfWorkManager>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_repo,
            uow_manager,
            password_hasher,
        }
    }

    pub async fn execute(
        &self,
        command: RegisterTenantCommand,
    ) -> Result<(Tenant, User), DomainError> {
        // 1. Regra de Negócio: E-mail único (Validação rápida, fora da transação)
        if self
            .user_repo
            .find_by_email(&command.admin_email)
            .await?
            .is_some()
        {
            return Err(DomainError::UserAlreadyExists);
        }

        // 2. Geração do Slug (Ex: "Minha Empresa LTDA" -> "minha-empresa-ltda")
        let slug = command.tenant_name.to_lowercase().replace(" ", "-");

        // 3. Construção das Entidades do Domínio
        let tenant = Tenant::new(command.tenant_name, slug);
        let user = User::new(command.admin_email, command.admin_full_name);

        // 4. Segurança e Vínculos
        let hashed_password = self.password_hasher.hash(&command.admin_plain_password)?;
        let credential = Credential::new(user.id, hashed_password);

        let relation = TenantUser::new(tenant.id, user.id, Role::Admin);

        // ==========================================
        // 5. O VERDADEIRO UNIT OF WORK EM AÇÃO
        // ==========================================

        // Inicia a transação genérica no banco de dados
        let mut uow = self.uow_manager.begin().await?;

        // Acessa os repositórios que estão "amarrados" a essa transação específica
        uow.tenants().create(&tenant).await?;
        uow.users().create(&user).await?;
        uow.credentials().create(&credential).await?;
        uow.tenants().add_user_to_tenant(&relation).await?;

        // Se tudo deu certo, efetiva as mudanças no banco de dados
        uow.commit().await?;

        Ok((tenant, user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database::{PgUnitOfWorkManager, PgUserRepository};
    use crate::infrastructure::security::Argon2Hasher;
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    // Função auxiliar para conectar no banco de testes e rodar as migrations
    async fn setup_test_db() -> sqlx::PgPool {
        // Conecta especificamente no banco de testes que você criou no Docker
        let db_url = "postgres://postgres:postgres@localhost:5432/nexus_helpdesk_test";

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            .expect("Falha ao conectar no banco de dados de teste");

        // Roda as migrations automaticamente no banco de testes antes de cada run!
        // O macro procura a pasta `migrations` na raiz do seu projeto.
        sqlx::migrate!("../../migrations")
            .run(&pool)
            .await
            .expect("Falha ao rodar migrations no banco de teste");

        pool
    }

    #[tokio::test]
    async fn test_register_tenant_success_saves_all_data() {
        // 1. Setup: Inicializa o Banco e as Dependências (Infraestrutura)
        let pool = setup_test_db().await;

        let user_repo = Arc::new(PgUserRepository::new(pool.clone()));
        let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
        let password_hasher = Arc::new(Argon2Hasher::new());

        // Injeta a infraestrutura real no Caso de Uso Puro
        let use_case = RegisterTenantUseCase::new(user_repo.clone(), uow_manager, password_hasher);

        // Usamos um UUID no e-mail para garantir que o teste rode múltiplas vezes sem
        // colidir com o `UNIQUE` da tabela de usuários no banco compartilhado.
        let random_email = format!("admin_{}@empresa.com", Uuid::new_v4());

        let command = RegisterTenantCommand {
            tenant_name: "Nexus Corp LTDA".to_string(),
            admin_full_name: "Jane Doe".to_string(),
            admin_email: random_email.clone(),
            admin_plain_password: "SenhaSuperForte123!".to_string(),
        };

        // 2. Ação: Executa o fluxo
        let result = use_case.execute(command).await;

        // 3. Asserts: Verifica as regras de negócio
        assert!(result.is_ok(), "O caso de uso deveria ter retornado Ok");
        let (tenant, user) = result.unwrap();

        assert_eq!(tenant.name, "Nexus Corp LTDA");
        assert_eq!(tenant.slug, "nexus-corp-ltda");
        assert_eq!(user.full_name, "Jane Doe");
        assert_eq!(user.email, random_email);

        // 4. Verificação Extra de Persistência (Lendo direto do banco via Repo)
        let saved_user_in_db = user_repo.find_by_email(&random_email).await.unwrap();
        assert!(saved_user_in_db.is_some());
        assert_eq!(saved_user_in_db.unwrap().id, user.id);
    }

    #[tokio::test]
    async fn test_register_tenant_fails_if_email_already_exists() {
        // 1. Setup
        let pool = setup_test_db().await;
        let user_repo = Arc::new(PgUserRepository::new(pool.clone()));
        let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
        let password_hasher = Arc::new(Argon2Hasher::new());

        let use_case = RegisterTenantUseCase::new(user_repo.clone(), uow_manager, password_hasher);

        let shared_email = format!("duplicate_{}@empresa.com", Uuid::new_v4());

        let command_1 = RegisterTenantCommand {
            tenant_name: "Empresa Um".to_string(),
            admin_full_name: "Bob".to_string(),
            admin_email: shared_email.clone(),
            admin_plain_password: "123".to_string(),
        };

        let command_2 = RegisterTenantCommand {
            tenant_name: "Empresa Dois".to_string(),
            admin_full_name: "Alice".to_string(),
            admin_email: shared_email.clone(), // Mesmo e-mail!
            admin_plain_password: "456".to_string(),
        };

        // 2. Ação
        let first_attempt = use_case.execute(command_1).await;
        let second_attempt = use_case.execute(command_2).await;

        // 3. Asserts
        assert!(first_attempt.is_ok(), "A primeira criação deve funcionar");

        assert!(second_attempt.is_err(), "A segunda criação deve falhar");
        match second_attempt.unwrap_err() {
            DomainError::UserAlreadyExists => {} // Sucesso! Caiu no erro esperado do domínio
            _ => panic!("Retornou o erro errado!"),
        }
    }
}
