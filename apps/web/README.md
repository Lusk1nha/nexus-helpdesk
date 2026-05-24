# @nexus/web (Tenant Workspace)

React frontend for the Nexus Helpdesk platform — multi-tenant, AI-powered, realtime.

> **Author:** Lucas Pedro · [github.com/Lusk1nha](https://github.com/Lusk1nha)

---

## 🎯 App Scope & Subdomains

This application serves as the **Tenant Workspace**. It is strictly designed to run under a tenant-specific subdomain (e.g., `apple.nexus.com` in production, or `apple.nexus.test` in development).

**What this app DOES NOT do:**
It does not handle the registration of new companies (Tenants). That responsibility belongs to `apps/onboarding`. This app assumes the Tenant already exists and operates securely within that bounded context.

| App                   | Domain               | What lives there                                                             |
| --------------------- | -------------------- | ---------------------------------------------------------------------------- |
| `apps/onboarding`     | `nexus.com`          | Landing page + tenant registration (creates company, redirects to workspace) |
| **`apps/web`**        | `[tenant].nexus.com` | **This app.** Login, tickets, knowledge, tenant-local admin                  |
| `apps/admin` _(soon)_ | `admin.nexus.com`    | Super-admin backoffice (auditing, billing, RAG monitoring)                   |

**SSO** between subdomains uses an **httpOnly cookie** scoped to `.nexus.com` (and `.nexus.test` in dev). The access token lives in memory; the refresh token lives in the cookie and is sent automatically on every request.

---

## Stack

| Layer        | Technology                                       |
| ------------ | ------------------------------------------------ |
| Framework    | React 19 + Vite 8                                |
| Language     | TypeScript 6                                     |
| Styling      | Tailwind CSS v4 (`text-(--var)` shorthand)       |
| Font         | JetBrains Mono Variable                          |
| Headless UI  | Base UI (`@base-ui/react`) — Button, Input, etc. |
| Icons        | Phosphor (`@phosphor-icons/react`)               |
| Routing      | React Router v7 (data-driven config)             |
| Server state | TanStack Query v5 (via `@nexus/query`)           |
| Client state | Zustand v5 (via `@nexus/auth`)                   |
| Validation   | Zod                                              |
| Forms        | React Hook Form + Zod resolver                   |
| HTTP         | ky + cookie-based silent refresh                 |
| Animations   | Motion (Framer Motion for React 19)              |
| Unit tests   | Vitest + React Testing Library                   |
| E2E tests    | Playwright _(coming)_                            |

---

## Monorepo packages

This app is one of several inside the Nexus monorepo (Turbo + pnpm workspaces). It consumes the following shared packages:

| Package           | Purpose                                                                                                                                                                      |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@nexus/ui`       | Design-system primitives built on Base UI: `Button`, `Input`, `Label`, `FormField`, `FormError`, `Alert{Title,Description}`, `Avatar{Image,Fallback,Badge,Group,GroupCount}` |
| `@nexus/theme`    | 19-theme registry, `ThemeProvider`, `ThemeSwitcher` (tabbed dark/light + showcase autoplay), all theme CSS variables                                                         |
| `@nexus/api`      | `createApiClient()` factory (ky + cookie-based silent refresh), `API` URL constants, `NexusApiError` typed errors, `fetchApi()` envelope unwrapper                           |
| `@nexus/auth`     | Auth types (`User`, `Role`, `LoginResult`, …), Zod schemas (`loginSchema`, `registerSchema`, `tenantSlugSchema`), and `useAuthStore` (Zustand)                               |
| `@nexus/query`    | TanStack Query `QueryProvider`                                                                                                                                               |
| `@nexus/utils`    | Pure helpers: `cn()`, `generateSlug()`                                                                                                                                       |
| `@nexus/tsconfig` | Shared TypeScript bases (`base`, `react`, `react-library`)                                                                                                                   |

---

## Architecture — Domain-Driven Design

```text
src/
├── application/            # Use cases as React hooks
│   └── auth/
│       ├── use-login.ts
│       └── use-session.ts
│
├── infrastructure/         # External adapters
│   ├── http/
│   │   └── client.ts       # Thin wrapper around @nexus/api's createApiClient
│   └── sse/                # Server-Sent Events client (coming)
│
└── presentation/           # UI layer
    ├── layouts/
    │   ├── auth.layout.tsx # Public pages (redirects to /app if authenticated)
    │   └── app.layout.tsx  # Protected pages (redirects to /login otherwise)
    ├── pages/
    │   ├── auth/
    │   │   ├── login.page.tsx       # ← Tenant-local login
    │   │   └── routes.tsx           # ← auth routes config (data-driven)
    │   └── app/
    │       ├── dashboard.page.tsx
    │       └── routes.tsx           # ← app routes config (data-driven)
    └── router/
        ├── types.ts                 # AppRoute = RouteObject + requiredRole
        ├── guards.tsx               # <RequireRole>
        ├── compose.tsx              # config → RouteObject[] (auto-wraps guards)
        ├── paths.ts                 # All URLs in one place (segments + paths)
        ├── routes.tsx               # Top-level layout composition
        └── index.ts
```

**Notice what's NOT here anymore:**

- `domain/auth/` → moved to `@nexus/auth` (types + Zod schemas, shared with `apps/onboarding`)
- `infrastructure/store/` → moved to `@nexus/auth` (`useAuthStore`)
- `infrastructure/http/api.routes.ts` → moved to `@nexus/api` (`API`)
- `presentation/components/{ui,theme}/` → moved to `@nexus/ui` and `@nexus/theme`
- `presentation/pages/auth/register.page.tsx` → moved to `apps/onboarding`
- `presentation/providers/query.provider.tsx` → moved to `@nexus/query`

**Adding a feature:** domain types → Zod schema (in `@nexus/auth` for cross-app, or feature-local for app-specific) → application hook → page component → one entry in the matching `routes.tsx` → one entry in `router/paths.ts` if it's a new URL.

---

## Theme System

19 built-in themes, infinitely extensible. The `ThemeSwitcher` shows them in two tabs (dark / light) with a "showcase mode" autoplay button that cycles random themes every 2 seconds:

| Mode  | Themes                                                                                                                                                          |
| ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Dark  | `midnight`, `dracula`, `nord`, `catppuccin`, `rose-pine`, `cyberpunk`, `forest`, `tokyo-night`, `oled-black`, `synthwave`, `night-runner`, `terminal`, `outrun` |
| Light | `dawn`, `paper`, `slate-light`, `serene`, `ice`, `coffee`                                                                                                       |

**Adding a new theme — two files in `packages/theme/`:**

1. Add a `[data-theme="my-theme"]` block in `packages/theme/src/theme.css` with all CSS variables (see existing themes for the full list — `--bg`, `--surface`, `--surface-2`, `--surface-hover`, `--border`, `--fg`, `--muted`, `--accent`, `--accent-fg`, `--accent-hover`, `--ring`, `--success`, `--warning`, `--destructive`, `--destructive-fg`).

2. Add an entry to `packages/theme/src/themes.ts`:
   ```ts
   { id: "my-theme", name: "My Theme", description: "...", isDark: true, accentHex: "#..." }
   ```

The `ThemeSwitcher` picks it up automatically. Theme preference is persisted in `localStorage` by `ThemeProvider`.

---

## Routes — Data-driven config

Routes are declared as **plain objects**, not JSX trees. The root `App` component is one line: `useRoutes(compose(routes))`. Each section's routes live next to its pages (`pages/<section>/routes.tsx`), and `router/paths.ts` is the single source of truth for every URL.

### Adding a new page

1. Create the page component: `src/presentation/pages/<section>/<name>.page.tsx`
2. Register the segment + absolute path in `router/paths.ts`
3. Add one entry to that section's `routes.tsx`:
   ```tsx
   { path: segments.myPage, element: <MyPage /> }
   ```

### Restricting by role (declarative)

Add `requiredRole` to the route. The `compose()` helper wraps the element in `<RequireRole>` automatically:

```tsx
{ path: segments.admin, element: <AdminPage />, requiredRole: "admin" }
{ path: segments.knowledge, element: <KnowledgePage />, requiredRole: ["admin", "agent"] }
```

### Current map

```
/                → redirect to /login
/login           → LoginPage       (public, redirects if authenticated)
/app             → AppLayout       (auth required)
  /app/tickets   → DashboardPage   (placeholder)
```

> **Registration** is **not** in this app — it lives in `apps/onboarding`, which creates the tenant and then redirects the user to `https://[slug].nexus.com/login`.

---

## Auth — cookie-based, in-memory access token

The backend issues two JWTs on login:

1. **Access token** — short-lived (15 min), returned in the response body, kept **in memory** by `useAuthStore`. Sent on every request as `Authorization: Bearer <token>`.
2. **Refresh token** — long-lived (30 days), set as an **httpOnly cookie** (`nexus_refresh`) scoped to `.nexus.com` / `.nexus.test`. The browser sends it automatically on every request; JavaScript never sees it.

### Silent refresh

When any request returns `401`, `@nexus/api`'s `afterResponse` hook:

1. Calls `POST /api/v1/identity/refresh` with `credentials: "include"` (cookie travels automatically)
2. Stores the new access token via `setAccessToken`
3. Retries the original request with the new token
4. On failure (cookie expired/revoked), calls `onAuthFailure()` which clears the store and redirects to `/login`

### Why this matters

- **No localStorage XSS risk** — the refresh token can't be stolen by injected scripts.
- **Cross-subdomain SSO** — log in once at `acme.nexus.com`, the same session works at every tenant-scoped subdomain.
- **Reload-safe** — `useAuthStore` persists only `user` to localStorage for instant shell rendering; the access token is rehydrated via silent refresh on the first authenticated request.

---

## HTTP Client

`src/infrastructure/http/client.ts` is a 13-line wrapper that wires this app's `useAuthStore` + `paths.login` into `@nexus/api`'s `createApiClient()` factory. Every app (`web`, `onboarding`, future `admin`) does this same wiring with its own redirect target.

```ts
export const http = createApiClient({
  baseUrl: env.apiUrl,
  getAccessToken: () => useAuthStore.getState().accessToken,
  setAccessToken: (token) => useAuthStore.getState().setAccessToken(token),
  onAuthFailure: () => {
    useAuthStore.getState().clear()
    window.location.href = paths.login
  },
})
```

### Typed errors

`fetchApi()` unwraps the `{ data, meta }` envelope and converts HTTP errors into a typed `NexusApiError` with `{ code, message, status }`, so React Query mutations get a clean `error.message` instead of raw `HTTPError`.

---

## Realtime (SSE) — _coming_

Server-Sent Events from the Rust backend:

- `GET /api/v1/realtime/tickets/:id` — per-ticket events (`message_added`, `status_changed`)
- `GET /api/v1/realtime/system` — system events for agents/admins (`ticket_created`, `ticket_status_changed`)

SSE hooks will live in `src/infrastructure/sse/` and `src/application/`. Because `EventSource` does not support custom headers, the JWT is passed as a query param (`?token=`). The backend SSE middleware accepts this on those two endpoints.

---

## Dev Setup

```bash
# Install all workspace deps (from repo root)
pnpm install

# Start the workspace dev server
pnpm --filter web dev          # http://localhost:5173

# Start the onboarding app dev server (separate terminal)
pnpm --filter onboarding dev   # http://localhost:5174

# Start the backend (Rust)
cargo run                      # http://localhost:8080

# Type-check the whole monorepo (Turbo, fully cached on re-runs)
pnpm type-check

# Production build
pnpm build
```

### Environment

Create `apps/web/.env.local`:

```env
VITE_API_URL=http://localhost:8080
```

### Subdomain testing in dev

Add to `/etc/hosts`:

```
127.0.0.1 acme.nexus.test onboarding.nexus.test admin.nexus.test
```

And set on the backend `.env`:

```env
COOKIE_DOMAIN=.nexus.test
COOKIE_SECURE=false
FRONTEND_URL=http://acme.nexus.test:5173,http://onboarding.nexus.test:5174
```

Then access `http://acme.nexus.test:5173` — the cookie set on login will work across all `*.nexus.test` subdomains.

---

## Testing Strategy

| Layer       | Tool                  | What                                                     |
| ----------- | --------------------- | -------------------------------------------------------- |
| Unit        | Vitest                | Domain schemas, guards, utility functions, store actions |
| Integration | RTL + MSW             | Form flows, hook behavior with mocked API                |
| E2E         | Playwright _(coming)_ | Full auth/ticket flows in a real browser                 |

---

## Code Style

Formatted with **Prettier** + `prettier-plugin-tailwindcss` (auto-sorts Tailwind classes and arguments to `cn()` / `cva()`).

```bash
pnpm format          # write
pnpm format:check    # CI-friendly check
```

Config lives at the repo root in `.prettierrc` and applies to the whole monorepo.

---

## Author

Built by **Lucas Pedro** — [github.com/Lusk1nha](https://github.com/Lusk1nha)

If you find this project useful, feel free to star ⭐ the repo or open an issue with feedback.
