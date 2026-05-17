-- 1. Tabela Principal de Tickets
CREATE TABLE tickets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES users(id), -- Quem abriu o chamado
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'open', -- open, processing_ai, awaiting_agent_approval, resolved, closed
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Trigger para manter o updated_at atualizado
CREATE TRIGGER trigger_tickets_updated_at
BEFORE UPDATE ON tickets
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- Índices essenciais para listagem rápida no painel do agente
CREATE INDEX idx_tickets_tenant_status ON tickets(tenant_id, status);
CREATE INDEX idx_tickets_customer ON tickets(customer_id);

-- 2. Tabela de Mensagens do Ticket (O Chat / RAG Context)
CREATE TABLE ticket_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    sender_id UUID, -- Pode ser nulo se quem enviou foi a IA
    sender_type VARCHAR(50) NOT NULL, -- 'customer', 'agent', 'ai', 'system'
    content TEXT NOT NULL,
    is_internal_note BOOLEAN NOT NULL DEFAULT FALSE, -- Notas invisíveis para o cliente
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    -- Mensagens geralmente são imutáveis (append-only), então não precisamos de updated_at
);

CREATE INDEX idx_ticket_messages_ticket_id ON ticket_messages(ticket_id);