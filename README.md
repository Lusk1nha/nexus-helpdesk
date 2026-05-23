# 🌌 Nexus Helpdesk (Agentic AI Support)

![Status](https://img.shields.io/badge/Status-Em%20Desenvolvimento-blue)
![Rust](<https://img.shields.io/badge/Backend-Rust%20(Axum)-orange>)
![React](https://img.shields.io/badge/Frontend-React%2019-61DAFB)
![AI](<https://img.shields.io/badge/AI-Ollama%20(Local)-000000>)

O **Nexus Helpdesk** é uma plataforma SaaS Multi-Tenant B2B focada em suporte ao cliente, construída desde o dia zero com uma arquitetura **Local-First** e **Inteligência Artificial (RAG) nativa**.

## 📖 A História e Motivação do Projeto

Este projeto nasceu da evolução natural após a construção de sistemas desktop de alta performance (como players de áudio com processamento neural local em C++/Rust). O objetivo principal do Nexus Helpdesk é resolver três grandes desafios de engenharia de software avançada:

1. **Domain-Driven Design (DDD) & Multi-Tenancy:** Criar um sistema onde os dados de múltiplas empresas (Tenants) coexistem no mesmo banco de dados relacional com isolamento total e segurança, utilizando o padrão **Unit of Work** compartilhado.
2. **Arquitetura Orientada a Eventos (Event-Driven):** Lidar com fluxos assíncronos onde a entrada de um ticket não bloqueia o sistema, sendo processada em background por _Workers_ de alta performance (`tokio::mpsc`).
3. **Privacidade e Custo-Zero (Local-First AI):** Em vez de depender de APIs caras na nuvem (como OpenAI), o sistema utiliza LLMs rodando 100% localmente via **Ollama**, garantindo que dados sensíveis de clientes nunca saiam da infraestrutura da empresa.

---

## 🚀 Tecnologias e Stack

A stack foi cuidadosamente selecionada para garantir performance máxima no processamento de dados e uma experiência fluida no frontend, isolando backend (`crates`) e frontend (`apps` e `packages`) em um ambiente de Monorepo via **Turborepo + pnpm workspaces**.

### 🦀 Core & Backend (Rust)
- **Axum:** Framework web assíncrono de altíssima performance para a API REST.
- **SQLx:** ORM e Query Builder com validação de queries SQL em tempo de compilação.
- **Tokio (`mpsc` & `Mutex`):** Gerenciamento de filas em memória e controle de concorrência para os _Workers_ de IA e transações do banco.
- **Clean Architecture:** Divisão estrita de responsabilidades (Ports, Adapters, Use Cases).

### 🧠 Motor de Inteligência Artificial (Local)
- **Ollama:** Orquestrador de LLMs rodando localmente (Modelos: `Phi-3` ou `Llama-3`).
- **Qdrant:** Banco de dados vetorial de altíssima performance (escrito em Rust) para armazenar os _embeddings_ da base de conhecimento (RAG).
- **Embeddings:** Modelo `nomic-embed-text` para busca semântica de documentos.

### ⚛️ Frontend & Ecossistema (React)
- **React 19 + TypeScript + Vite:** Base das interfaces SPA.
- **Zustand & TanStack Query:** Gerenciamento de estado global e cache assíncrono.
- **Tailwind CSS v4 + Shadcn/UI:** Sistema de design e componentes.
- **SSO & Cookies:** Sessão cross-subdomínio ancorada no domínio principal (`.nexus.com`).

---

## 🏗️ Arquitetura do Monorepo

O projeto é estritamente modularizado em contextos delimitados, abrangendo tanto a camada de dados quanto a de apresentação.

### 🦀 Crates (Backend Rust)
1. **`shared_kernel`**: Infraestrutura comum (ex: `DatabaseConnection` para controle do **Unit of Work**).
2. **`domain_identity`**: Gerenciamento de Tenants (empresas), usuários, RBAC e JWT.
3. **`domain_ticketing`**: Máquina de estados dos chamados e **Workers de IA** que operam de forma assíncrona.
4. **`api_gateway`**: Porta de entrada HTTP (Axum), injeção de dependências (`AppState`) e extratores de subdomínio.
5. **`ai_engine`**: Interface com LLMs locais e banco vetorial (Qdrant).

### 🌐 Apps (Fronteiras de Apresentação)
Para evitar sobreposição de responsabilidades e otimizar o roteamento, o frontend é dividido em três aplicações isoladas:
1. **`apps/onboarding` (Global):** Porta de entrada (`app.nexus.localhost`). Responsável por Landing Pages, escolha do *slug* (nome do subdomínio) e criação de novas empresas.
2. **`apps/web` (Tenant-Local):** O workspace diário (`[tenant].nexus.localhost`). Funciona apenas no contexto de uma empresa já existente. Contém a fila de tickets dos agentes e as telas de autoatendimento para clientes finais.
3. **`apps/admin` (Backoffice Global):** Área restrita para os mantenedores da plataforma (gestão de tenants, uso de recursos e auditoria).

### 📦 Packages (Bibliotecas Internas)
- **`@nexus/ui`**: Sistema de design primário (Botões, Inputs, Modais).
- **`@nexus/theme`**: Sistema avançado de multi-temas injetável (CSS puro + tokens).
- **`@nexus/auth`**: Lógica compartilhada de hooks de sessão.

---

## 🗺️ Roadmap de Desenvolvimento e Status

### ✅ Fase 1: Arquitetura Base e Infraestrutura de Dados
- [x] Configuração do Monorepo (Cargo Workspaces + pnpm + Turborepo).
- [x] Modelagem do banco de dados relacional para Multi-Tenancy.
- [x] Implementação do padrão **Unit of Work (UoW)** e repositórios genéricos com `SQLx`.
- [x] Tratamento avançado de concorrência de transações de banco (`Shared Kernel`).

### ✅ Fase 2: Motor de Tarefas e IA Base
- [x] Criação do **AiWorker** rodando em background.
- [x] Comunicação via canais `tokio::mpsc` entre a API e os Workers.
- [x] Integração HTTP com a LLM local (Ollama).
- [x] Máquina de estados robusta (Fallback de falha da IA e rollback de banco automático).

### ⏳ Fase 3: Gateway e Rotas HTTP (Em Andamento)
- [ ] Criar os _Handlers_ do Axum mapeando para os _Use Cases_.
- [ ] Implementar middleware de autenticação (SSO Cookies/JWT).
- [ ] Implementar extrator de Subdomínio no Axum (identificar requisições vindas de `*.nexus.localhost`).

### ✅ Fase 4: Core de IA Avançado e RAG
- [x] Criar crate `ai_engine` para centralizar lógica vetorial.
- [x] Gerar _embeddings_ via Ollama e injetar no Qdrant.
- [x] Implementar busca de contexto semântico (_Retrieval_) para o `AiWorker`.
- [x] Endpoint para agentes/admins injetarem artigos na base de conhecimento.

### 🔜 Fase 5: Ecossistema Frontend
- [ ] Migrar layout atual para estrutura isolada (`apps/web` e `apps/onboarding`).
- [ ] Roteamento dinâmico baseado em Tenant slug (`[tenant].localhost`).
- [ ] Sincronização em tempo real via Server-Sent Events (SSE).

---

## 🛠️ Configuração do Ambiente de Desenvolvimento

### Pré-requisitos
- [Rust](https://www.rust-lang.org/tools/install) (cargo, rustc)
- [Node.js](https://nodejs.org/) & [PNPM](https://pnpm.io/)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Ollama](https://ollama.com/) (Instalado e rodando)

### Passos para Rodar

1. **Preparar a Inteligência Artificial:**
   Faça o pull dos modelos no seu terminal:
```bash
   ollama run phi3
   ollama pull nomic-embed-text