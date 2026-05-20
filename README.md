# 🌌 Nexus Helpdesk (Agentic AI Support)

![Status](https://img.shields.io/badge/Status-Em%20Desenvolvimento-blue)
![Rust](https://img.shields.io/badge/Backend-Rust%20(Axum)-orange)
![React](https://img.shields.io/badge/Frontend-React%2019-61DAFB)
![AI](https://img.shields.io/badge/AI-Ollama%20(Local)-000000)

O **Nexus Helpdesk** é uma plataforma SaaS Multi-Tenant B2B focada em suporte ao cliente, construída desde o dia zero com uma arquitetura **Local-First** e **Inteligência Artificial (RAG) nativa**. 

## 📖 A História e Motivação do Projeto

Este projeto nasceu da evolução natural após a construção de sistemas desktop de alta performance (como players de áudio com processamento neural local em C++/Rust). O objetivo principal do Nexus Helpdesk é resolver três grandes desafios de engenharia de software avançada:

1. **Domain-Driven Design (DDD) & Multi-Tenancy:** Criar um sistema onde os dados de múltiplas empresas (Tenants) coexistem no mesmo banco de dados relacional com isolamento total e segurança, utilizando o padrão **Unit of Work** compartilhado.
2. **Arquitetura Orientada a Eventos (Event-Driven):** Lidar com fluxos assíncronos onde a entrada de um ticket não bloqueia o sistema, sendo processada em background por *Workers* de alta performance (`tokio::mpsc`).
3. **Privacidade e Custo-Zero (Local-First AI):** Em vez de depender de APIs caras na nuvem (como OpenAI), o sistema utiliza LLMs rodando 100% localmente via **Ollama**, garantindo que dados sensíveis de clientes nunca saiam da infraestrutura da empresa.

---

## 🚀 Tecnologias e Stack

A stack foi cuidadosamente selecionada para garantir performance máxima no processamento de dados e uma experiência fluida no frontend, isolando backend (`crates`) e frontend (`apps`) em um ambiente de Monorepo (Cargo Workspace + PNPM).

### 🦀 Core & Backend (Rust)
- **Axum:** Framework web assíncrono de altíssima performance para a API REST.
- **SQLx:** ORM e Query Builder com validação de queries SQL em tempo de compilação.
- **Tokio (`mpsc` & `Mutex`):** Gerenciamento de filas em memória e controle de concorrência para os *Workers* de IA e transações do banco.
- **Clean Architecture:** Divisão estrita de responsabilidades (Ports, Adapters, Use Cases).

### 🧠 Motor de Inteligência Artificial (Local)
- **Ollama:** Orquestrador de LLMs rodando localmente (Modelos: `Phi-3` ou `Llama-3`).
- **Qdrant:** Banco de dados vetorial de altíssima performance (escrito em Rust) para armazenar os *embeddings* da base de conhecimento (RAG).
- **Embeddings:** Modelo `nomic-embed-text` para busca semântica de documentos.

### ⚛️ Workspace do Agente (Frontend)
- **React 19 + TypeScript + Vite:** Base da interface SPA.
- **Zustand:** Gerenciamento de estado global leve (sessões e estado da interface).
- **TanStack Query:** Sincronização assíncrona, cache e chamadas otimizadas para a API.
- **Tailwind CSS + Shadcn/UI:** Componentes de interface modernos, responsivos e acessíveis.

---

## 🏗️ Arquitetura de Domínios (Módulos)

O projeto é dividido em *crates* focados em contextos delimitados:

1. **`shared_kernel`**: Infraestrutura comum a todo o sistema, como o **DatabaseConnection** que permite o compartilhamento dinâmico de transações via `Arc<Mutex<Option<Transaction>>>`.
2. **`domain_identity`**: Gerencia a autenticação (JWT), locatários (Tenants) e controle de acesso baseado em funções (RBAC).
3. **`domain_ticketing`**: A máquina de estados dos tickets. Contém os Casos de Uso e o **Worker de IA** que processa chamados em background sem travar a thread principal.
4. **`api_gateway`**: A porta de entrada HTTP. Contém as rotas Axum, o `AppState` (Injeção de Dependências) e os Middlewares.

---

## 🗺️ Roadmap de Desenvolvimento e Status

### ✅ Fase 1: Arquitetura Base e Infraestrutura de Dados
- [x] Configuração do Monorepo (Cargo Workspaces + pnpm).
- [x] Modelagem do banco de dados relacional para Multi-Tenancy.
- [x] Implementação do padrão **Unit of Work (UoW)** e repositórios genéricos com `SQLx`.
- [x] Tratamento avançado de concorrência de transações de banco (`Shared Kernel`).

### ✅ Fase 2: Motor de Tarefas e IA Base
- [x] Criação do **AiWorker** rodando em background.
- [x] Comunicação via canais `tokio::mpsc` entre a API e os Workers.
- [x] Integração HTTP via `reqwest` com a LLM local (Ollama).
- [x] Máquina de estados robusta (Fallback de falha da IA e rollback de banco automático).
- [x] Construção do `AppState` injetando dependências nos *Use Cases*.

### ⏳ Fase 3: Gateway e Rotas HTTP (Próximo Passo)
- [ ] Criar os *Handlers* do Axum (receber requisições e mapear para os *Use Cases*).
- [ ] Implementar middleware de autenticação (validação de JWT).
- [ ] Implementar extrator de Tenant (descobrir de qual empresa é a requisição).

### ✅ Fase 4: Core de IA Avançado e RAG
- [x] Criar crate `ai_engine` para centralizar lógica vetorial.
- [x] Gerar *embeddings* via Ollama (`nomic-embed-text`) e injetar no banco de dados vetorial (Qdrant).
- [x] Implementar busca de contexto semântico (*Retrieval*) para enviar ao `AiWorker` antes de responder o ticket.
- [x] Indexar tickets resolvidos automaticamente ao aprovar resposta da IA.
- [x] Endpoint `POST /api/v1/knowledge` para agentes/admins injetarem artigos na base de conhecimento.

### 🔜 Fase 5: Frontend e UX
- [ ] Configuração do cliente do `TanStack Query`.
- [ ] Gerenciamento de sessão com `Zustand`.
- [ ] Construção do painel do Agente com fila em tempo real.

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