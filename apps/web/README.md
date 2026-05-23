# @nexus/web (Tenant Workspace)

React frontend for the Nexus Helpdesk platform — multi-tenant, AI-powered, realtime.

> **Author:** Lucas Pedro · [github.com/Lusk1nha](https://github.com/Lusk1nha)

---

## 🎯 App Scope & Subdomains

This application serves as the **Tenant Workspace**. It is strictly designed to run under a tenant-specific subdomain (e.g., `apple.nexus.com` in production, or `apple.localhost` in development). 

**What this app DOES NOT do:**
It does not handle the registration of new companies (Tenants). That responsibility belongs to `apps/onboarding`. This app assumes the Tenant already exists and operates securely within that bounded context.

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

| Package           | Purpose                                                                               |
| ----------------- | ------------------------------------------------------------------------------------- |
| `@nexus/ui`       | Design system primitives — `Button`, `Input`, `Label`, `FormField`, `Alert`, `cn()`   |
| `@nexus/theme`    | Multi-theme system — `ThemeProvider`, `ThemeSwitcher`, theme registry, theme CSS vars |
| `@nexus/auth`     | Shared auth session logic and SSO token management                                    |
| `@nexus/tsconfig` | Shared TypeScript bases (`base`, `react`, `react-library`)                            |

---

## Architecture — Domain-Driven Design

```text
src/
├── domain/                 # Pure business logic — no framework deps
│   ├── auth/
│   │   ├── auth.types.ts   # User, Role interfaces
│   │   └── auth.schemas.ts # Zod validation schemas (Login only)
│   └── ticketing/          # Ticket, Message types + schemas
│
├── application/            # Use cases as React hooks
│   └── auth/
│       ├── use-login.ts
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
    ├── layouts/
    │   ├── auth.layout.tsx # Public pages (redirects if authenticated)
    │   └── app.layout.tsx  # Protected pages (redirects if not authenticated)
    ├── pages/
    │   ├── auth/
    │   │   ├── login.page.tsx  # ← Tenant-local login
    │   │   └── routes.tsx      
    │   └── app/
    │       ├── dashboard.page.tsx
    │       └── routes.tsx      
    ├── providers/
    │   └── query.provider.tsx
    └── router/
        ├── types.ts        # AppRoute type (extends RouteObject + requiredRole)
        ├── guards.tsx      # <RequireRole> component
        ├── compose.tsx     # config → RouteObject[] transform
        ├── routes.tsx      # top-level layout composition
        └── index.ts