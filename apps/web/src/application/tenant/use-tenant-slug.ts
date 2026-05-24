/**
 * Reserved subdomains that are not tenant slugs.
 * Accessing admin.nexus.localhost via the web app should not be treated
 * as a tenant named "admin".
 */
const RESERVED = new Set(["onboarding", "admin", "www", "api"])

function getSlugFromHostname(): string | null {
  const { hostname } = window.location

  // Plain "localhost" or an IP — no subdomain at all.
  if (!hostname.includes(".")) return null

  const [candidate] = hostname.split(".")
  if (!candidate || RESERVED.has(candidate)) return null

  return candidate
}

/**
 * Returns the active tenant slug for this browser session.
 *
 * Resolution order:
 *  1. VITE_TENANT_SLUG env var — lets you develop without subdomains:
 *       VITE_TENANT_SLUG=acme pnpm dev
 *  2. First subdomain of the current hostname:
 *       acme.nexus.localhost  →  "acme"
 *       localhost             →  null
 *
 * A null return means the user reached the app without a valid tenant
 * context — the NoTenantPage guard will handle it.
 */
export function useTenantSlug(): string | null {
  return (import.meta.env.VITE_TENANT_SLUG as string | undefined) ?? getSlugFromHostname()
}
