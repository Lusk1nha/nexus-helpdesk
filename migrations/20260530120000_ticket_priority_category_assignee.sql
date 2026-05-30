-- Triagem de tickets: prioridade, categoria e responsável (assignee).
--
-- priority  : low | normal | high  (default 'normal' para tickets já existentes)
-- category  : rótulo livre opcional (ex.: "billing", "bug", "account")
-- assignee_id: agente que "assumiu" o chamado (NULL = sem responsável)

ALTER TABLE tickets
    ADD COLUMN priority VARCHAR(20) NOT NULL DEFAULT 'normal',
    ADD COLUMN category VARCHAR(50),
    ADD COLUMN assignee_id UUID REFERENCES users(id) ON DELETE SET NULL;

-- Índices para o painel do agente (filtrar por prioridade / "atribuídos a mim").
CREATE INDEX idx_tickets_tenant_priority ON tickets(tenant_id, priority);
CREATE INDEX idx_tickets_assignee ON tickets(assignee_id);
