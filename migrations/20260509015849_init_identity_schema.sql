-- Habilita a geração nativa de UUIDs no Postgres
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Função automática para atualizar a coluna updated_at
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 1. Tabela de Usuários (Identidade)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    avatar_url TEXT,
    timezone VARCHAR(50) DEFAULT 'UTC', -- Vital para cálculo de SLA de chamados
    is_active BOOLEAN NOT NULL DEFAULT TRUE, -- Soft Delete
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER trigger_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- 2. Tabela de Credenciais (Segurança)
CREATE TABLE credentials (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    password_hash TEXT NOT NULL,
    failed_attempts INT NOT NULL DEFAULT 0, -- Trava de segurança contra força bruta
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER trigger_credentials_updated_at
BEFORE UPDATE ON credentials
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- 3. Tabela de Tenants (Multi-tenancy)
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL, -- Ex: "minha-empresa" (útil para URLs)
    plan VARCHAR(50) NOT NULL DEFAULT 'free', -- free, pro, enterprise (limita a IA)
    is_active BOOLEAN NOT NULL DEFAULT TRUE, -- Suspender a empresa inteira por falta de pgto
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER trigger_tenants_updated_at
BEFORE UPDATE ON tenants
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- 4. Vínculo de Usuários com Tenants (RBAC)
CREATE TABLE tenant_users (
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL, 
    is_active BOOLEAN NOT NULL DEFAULT TRUE, -- Usuário pode estar inativo em UM tenant, mas ativo em outro
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tenant_id, user_id)
);

CREATE TRIGGER trigger_tenant_users_updated_at
BEFORE UPDATE ON tenant_users
FOR EACH ROW EXECUTE FUNCTION set_updated_at();