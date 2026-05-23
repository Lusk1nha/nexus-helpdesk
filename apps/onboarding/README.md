# @nexus/onboarding (Provisioning Portal)

React frontend for the Nexus Helpdesk onboarding portal. This is the **Global Gateway** where companies begin their journey.

> **Author:** Lucas Pedro · [github.com/Lusk1nha](https://github.com/Lusk1nha)

---

## 🎯 App Scope & Subdomains

This application acts as the entry point for the entire platform. It runs under the root domain or a dedicated provisioning subdomain (e.g., `app.nexus.localhost` in development).

**Core Responsibilities:**

* **Landing Page:** Showcasing the project's vision, architecture, and local-first AI philosophy.
* **Tenant Provisioning:** Handling the registration flow where new companies define their workspace name, unique slug (subdomain), and administrator credentials.
* **Verification:** Real-time availability checks for workspace slugs.

**What this app DOES NOT do:**
It does not manage tickets, customers, or agents. Once a tenant is provisioned, this app performs a cross-subdomain navigation to the new workspace managed by `apps/web`.

---

## Stack

| Layer | Technology |
| --- | --- |
| Framework | React 19 + Vite 8 |
| Language | TypeScript 6 |
| Styling | Tailwind CSS v4 |
| Font | JetBrains Mono Variable |
| Routing | React Router v7 |
| Server state | TanStack Query v5 + `@nexus/query` |
| Validation | Zod |
| Forms | React Hook Form + Zod resolver |
| HTTP | ky (fetch wrapper) |
| Animations | Motion (Framer Motion) |
| Icons | Lucide React + Phosphor Icons |

---

## Monorepo Structure

This is one app inside the larger Nexus monorepo (Turbo + pnpm workspaces). It depends on shared packages:

| Package | Purpose |
| --- | --- |
| `@nexus/ui` | Design system primitives — `Button`, `Input`, `FormField`, `FormError`, `Alert`, `cn()` |
| `@nexus/theme` | Multi-theme system — `ThemeProvider`, `ThemeSwitcher`, theme CSS vars |
| `@nexus/auth` | Shared registration logic and validation schemas (`registerSchema`) |
| `@nexus/query` | Shared TanStack Query configuration and providers |
| `@nexus/api` | Shared HTTP client factory (`createApiClient`) |
| `@nexus/utils` | Shared utilities (e.g., `generateSlug`) |

---

## Architecture — Domain-Driven Design

```text
src/
├── application/            # Use cases (React hooks)
│   └── auth/
│       └── use-register.ts # Mutation hook for tenant provisioning
│
├── infrastructure/         # External adapters
│   └── http/
│       └── client.ts       # Public-only ky instance (no auth injection)
│
└── presentation/           # UI layer
    ├── layouts/
    │   └── onboarding.layout.tsx  # Shared grid-layout for public pages
    ├── components/
    │   ├── about/                 # About section blocks
    │   └── terminal-typing.tsx    # UI effects
    └── pages/
        ├── landing.page.tsx       # Marketing & Vision entry point
        └── register.page.tsx      # Multi-step provisioning form

```

## Setup

1. **Install Dependencies:**
```bash

```



pnpm install

```

2. **Environment Variables:**
   Ensure `VITE_API_URL` is set in your `.env.local` pointing to your Rust backend (usually `[http://api.localhost:8080](http://api.localhost:8080)`).

3. **Development:**
   ```bash
   pnpm dev

```

4. **Testing:**
```bash
pnpm test        # Run vitest
pnpm type-check  # Run tsc

```
