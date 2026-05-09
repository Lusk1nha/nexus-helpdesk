# 🌌 Nexus Helpdesk (Agentic AI Support)

![Status](https://img.shields.io/badge/Status-Em%20Desenvolvimento-blue)
![Rust](https://img.shields.io/badge/Backend-Rust%20(Axum)-orange)
![React](https://img.shields.io/badge/Frontend-React%2019-61DAFB)
![AI](https://img.shields.io/badge/AI-Ollama%20(Local)-000000)

O **Nexus Helpdesk** é uma plataforma SaaS Multi-Tenant B2B focada em suporte ao cliente, construída desde o dia zero com uma arquitetura **Local-First** e **Inteligência Artificial (RAG) nativa**. 

## 📖 A História e Motivação do Projeto

Este projeto nasceu da evolução natural após a construção de sistemas desktop de alta performance (como players de áudio com processamento neural local em C++/Rust). O objetivo principal do Nexus Helpdesk é resolver três grandes desafios de engenharia de software avançada:

1. **Domain-Driven Design (DDD) & Multi-Tenancy:** Criar um sistema onde os dados de múltiplas empresas (Tenants) coexistem no mesmo banco de dados relacional com isolamento total e segurança (Row-Level Security).
2. **Arquitetura Orientada a Eventos (Event-Driven):** Lidar com fluxos assíncronos onde a entrada de um ticket não bloqueia o sistema, sendo processada em background por *Workers* de alta performance.
3. **Privacidade e Custo-Zero (Local-First AI):** Em vez de depender de APIs caras na nuvem (como OpenAI), o sistema utiliza LLMs rodando 100% localmente via **Ollama**, garantindo que dados sensíveis de clientes nunca saiam da infraestrutura da empresa.

---

## 🚀 Tecnologias e Stack

A stack foi cuidadosamente selecionada para garantir performance máxima no processamento de dados e uma experiência fluida no frontend, tudo orquestrado via containers para facilitar o desenvolvimento local.

### 🦀 Core & Backend (Rust)
- **Axum:** Framework web assíncrono de altíssima performance para a API REST.
- **SQLx:** ORM e Query Builder com validação de queries SQL em tempo de compilação.
- **Tokio (`mpsc`):** Gerenciamento de filas em memória e concorrência para os *Workers* de IA.
- **Reqwest:** Para comunicação HTTP interna com o motor de Inteligência Artificial.

### 🧠 Motor de Inteligência Artificial (Local)
- **Ollama:** Orquestrador de LLMs rodando localmente (Modelos sugeridos: `Phi-3` ou `Llama-3`).
- **Qdrant:** Banco de dados vetorial de altíssima performance (escrito em Rust) para armazenar os *embeddings* da base de conhecimento (RAG).
- **Embeddings:** Modelo `nomic-embed-text` para busca semântica de documentos.

### ⚛️ Workspace do Agente (Frontend)
- **React 19 + TypeScript + Vite:** Base da interface SPA.
- **Zustand:** Gerenciamento de estado global leve (sessões e estado da interface).
- **TanStack Query:** Sincronização assíncrona, cache e chamadas otimizadas para a API.
- **Tailwind CSS + Shadcn/UI:** Componentes de interface modernos, responsivos e acessíveis.

### 🐳 Infraestrutura
- **Docker & Docker Compose:** Orquestração do PostgreSQL (Banco Relacional) e Qdrant (Banco Vetorial) sem poluir a máquina host.
- **PostgreSQL:** Fonte da verdade para usuários, permissões (RBAC) e tickets, focado no isolamento Multi-Tenant.

---

## 🏗️ Arquitetura de Domínios (Módulos)

O projeto é dividido em contextos delimitados:

1. **Identity & Access (IAM):** Gerencia a autenticação, separação de locatários (Tenants) e controle de acesso baseado em funções (Admin, Agente, Cliente).
2. **Ticketing Engine:** A máquina de estados dos tickets. Recebe chamados (simulados via API no MVP) e os enfileira para processamento assíncrono.
3. **RAG Pipeline (Knowledge Base):** Responsável por ingerir documentos locais (PDFs/TXTs), gerar vetores matemáticos e consultar o Qdrant para fornecer contexto à IA antes de responder a um ticket.
4. **Agent Workspace:** A interface visual onde o agente humano aprova respostas geradas pela IA, monitora a fila em tempo real e visualiza métricas de *Ticket Deflection*.

---

## 🛠️ Configuração do Ambiente de Desenvolvimento

### Pré-requisitos
- [Rust](https://www.rust-lang.org/tools/install) (cargo, rustc)
- [Node.js](https://nodejs.org/) & [PNPM](https://pnpm.io/)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Ollama](https://ollama.com/) (Instalado e rodando em background)

### Passos para Rodar

1. **Preparar a Inteligência Artificial:**
   Faça o pull dos modelos locais no seu terminal:
   ```bash
   ollama run phi3
   ollama pull nomic-embed-text
   

```

2. **Subir a Infraestrutura de Dados:**
Inicie o PostgreSQL e o Qdrant via Docker:
```bash
docker-compose up -d


```



```

3. **Rodar as Migrations do Banco:**
   ```bash
   cargo sqlx database setup

```

4. **Iniciar o Backend (Rust):**
```bash
cargo run


```



```

5. **Iniciar o Frontend (React):**
   Em um novo terminal, navegue até a pasta do painel e rode:
   ```bash
   cd apps/web
   pnpm install
   pnpm dev
   

```

---

## 🗺️ Roadmap de Desenvolvimento

* [ ] **Fase 1: Infraestrutura e Modelagem**
* Configurar `docker-compose.yml` (Postgres + Qdrant).
* Criar schema do banco para Multi-Tenancy.


* [ ] **Fase 2: Core de IA e RAG**
* Criar rotas no Rust para gerar embeddings via Ollama e salvar no Qdrant.


* [ ] **Fase 3: Motor Assíncrono**
* Implementar fila com `tokio::mpsc` para processamento de tickets em background.
* Criar o prompt do Agente Autônomo que lê a fila e gera respostas.


* [ ] **Fase 4: Frontend e UX**
* Painel do Agente com TanStack Query.
* Tela de chat com sugestões automáticas da IA.



---

## 📄 Licença

Desenvolvido com foco em engenharia de software avançada, estudos arquiteturais e privacidade de dados.

```

---

### Dica para organizar as pastas do novo projeto:

Já que você gostou do esquema de `crates` (monorepo), você pode organizar assim:

```text
nexus-helpdesk/
├── docker-compose.yml
├── .env
├── apps/
│   └── web/                # O Frontend React (Vite, Tailwind, etc)
└── crates/
    ├── api_gateway/        # Servidor web principal (Axum), rotas e Auth
    ├── domain_ticketing/   # Regras de negócio, máquina de estado, workers (Tokio)
    ├── domain_identity/    # Multi-tenancy, RBAC, acesso ao PostgreSQL (SQLx)
    └── ai_engine/          # Integração com Ollama (Reqwest) e RAG (Qdrant)
