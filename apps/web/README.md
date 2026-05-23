# nexus-helpdesk / web

React frontend for the Nexus Helpdesk platform — multi-tenant, AI-powered, realtime.

> **Author:** Lucas Pedro · [github.com/Lusk1nha](https://github.com/Lusk1nha)

---

## Stack

| Layer        | Technology                          |
| ------------ | ----------------------------------- |
| Framework    | React 19 + Vite 8                   |
| Language     | TypeScript 6                        |
| Styling      | Tailwind CSS v4                     |
| Font         | JetBrains Mono Variable             |
| Routing      | React Router v7                     |
| Server state | TanStack Query v5                   |
| Client state | Zustand v5                          |
| Validation   | Zod                                 |
| Forms        | React Hook Form + Zod resolver      |
| HTTP         | ky (fetch wrapper)                  |
| Animations   | Motion (Framer Motion for React 19) |
| Icons        | Lucide React                        |
| Unit tests   | Vitest + React Testing Library      |
| E2E tests    | Playwright _(coming)_               |

---

## Monorepo Structure

This is one app inside the larger Nexus monorepo (Turbo + pnpm workspaces). It depends on shared packages:

| Package | Purpose |
|---|---|
| `@nexus/ui` | Design system primitives — `Button`, `Input`, `Label`, `FormField`, `Alert`, `cn()` |
| `@nexus/theme` | Multi-theme system — `ThemeProvider`, `ThemeSwitcher`, theme registry, theme CSS vars |
| `@nexus/tsconfig` | Shared TypeScript bases (`base`, `react`, `react-library`) |

A future `apps/admin` (admin-only panel for auditing + RAG management) will reuse the same packages.

---

## Architecture — Domain-Driven Design

```
src/
├── domain/                 # Pure business logic — no framework deps
│   ├── auth/
│   │   ├── auth.types.ts   # User, Role, TokenPair interfaces
│   │   └── auth.schemas.ts # Zod validation schemas
│   └── ticketing/          # Ticket, Message types + schemas (coming)
│
├── application/            # Use cases as React hooks
│   └── auth/
│       ├── use-login.ts
│       ├── use-register.ts
│       └── use-session.ts
│
├── infrastructure/         # External adapters
│   ├── http/
│   │   ├── client.ts       # ky instance — auth injection + token refresh
│   │   └── api.routes.ts   # All API URL constants
│   └── store/
│       └── auth.store.ts   # Zustand store — session persistence
│
└── presentation/           # UI layer
    ├── components/
    │   ├── ui/             # Design system primitives (Button, Input, Label…)
    │   └── theme/          # ThemeSwitcher
    ├── layouts/
    │   ├── auth.layout.tsx # Public pages (redirects if authenticated)
    │   └── app.layout.tsx  # Protected pages (redirects if not authenticated)
    ├── pages/
    │   ├── auth/           # login.page.tsx, register.page.tsx
    │   └── app/            # dashboard, tickets, knowledge, admin (coming)
    ├── providers/
    │   ├── query.provider.tsx
    │   └── theme.provider.tsx
    └── theme/
        └── themes.ts       # Theme registry
```

**Adding a feature:** domain types → Zod schema → application hook → page component. No layer skips.

---

## Theme System

Five built-in themes, infinitely extensible:

| ID           | Name       | Style                 |
| ------------ | ---------- | --------------------- |
| `midnight`   | Midnight   | GitHub dark (default) |
| `dawn`       | Dawn       | Clean light           |
| `dracula`    | Dracula    | Classic vampire       |
| `nord`       | Nord       | Arctic blue           |
| `catppuccin` | Catppuccin | Soothing mocha        |

**Adding a new theme — two steps:**

1. Add a CSS block in `src/index.css`:

```css
[data-theme="my-theme"] {
  --bg: #...;
  --surface: #...;
  --surface-2: #...;
  --border: #...;
  --fg: #...;
  --muted: #...;
  --accent: #...;
  --accent-fg: #...;
  --success: #...;
  --warning: #...;
  --destructive: #...;
  --destructive-fg: #...;
}
```

2. Add an entry to `src/presentation/theme/themes.ts`:

```ts
{ id: 'my-theme', name: 'My Theme', description: '...', isDark: true, accentHex: '#...' }
```

The `ThemeSwitcher` picks it up automatically. Theme preference is persisted in `localStorage`.

---

## Routes

```
/                → redirect to /login
/login           → LoginPage       (public, redirects if authenticated)
/register        → RegisterPage    (public, redirects if authenticated)
/app             → AppLayout       (auth required)
  /app/tickets   → TicketListPage  (all roles)
  /app/knowledge → KnowledgePage   (agent, admin)
  /app/admin     → AdminLayout     (admin only)
```

---

## HTTP Client

`src/infrastructure/http/client.ts` wraps `ky` with:

- **Auth injection** — injects `Authorization: Bearer <token>` on every request via `beforeRequest` hook
- **Silent refresh** — on 401, calls `POST /identity/refresh`, retries the original request, and redirects to `/login` on failure

All responses are unwrapped via `fetchApi()` which strips the `{ data, meta }` envelope.

---

## Realtime (SSE)

Server-Sent Events from the Rust backend:

- `GET /api/v1/realtime/tickets/:id` — per-ticket events (`message_added`, `status_changed`)
- `GET /api/v1/realtime/system` — system events for agents/admins (`ticket_created`, `ticket_status_changed`)

SSE hooks live in `src/infrastructure/sse/` and `src/application/`. Because `EventSource` does not support custom headers, the JWT is passed as a query param (`?token=`). The backend SSE middleware accepts this on those two endpoints.

---

## Dev Setup

```bash
# Install all workspace deps (from repo root)
pnpm install

# Start frontend dev server
pnpm --filter web dev        # http://localhost:5173

# Start backend (separate terminal, from repo root)
cargo run

# Type check
pnpm --filter web exec tsc --noEmit

# Unit + integration tests
pnpm --filter web test

# Production build
pnpm --filter web build
```

### Environment

Create `apps/web/.env.local`:

```env
VITE_API_URL=http://localhost:8080
```

---

## Testing Strategy

| Layer       | Tool                  | What                                                     |
| ----------- | --------------------- | -------------------------------------------------------- |
| Unit        | Vitest                | Domain schemas, guards, utility functions, store actions |
| Integration | RTL + MSW             | Form flows, hook behavior with mocked API                |
| E2E         | Playwright _(coming)_ | Full auth/ticket flows in a real browser                 |

---

## Code Style

Formatted with **Prettier** + `prettier-plugin-tailwindcss` (auto-sorts Tailwind classes).

```bash
# Format the whole web app
pnpm --filter web format

# Check formatting (CI-friendly)
pnpm --filter web format:check
```

Config lives at the repo root in `.prettierrc.json` and applies to the whole monorepo.

---

## Author

Built by **Lucas Pedro** — [github.com/Lusk1nha](https://github.com/Lusk1nha)

If you find this project useful, feel free to star ⭐ the repo or open an issue with feedback.
