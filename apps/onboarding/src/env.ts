/**
 * Typed env access. Reading `import.meta.env.*` directly is forbidden outside
 * this file — go through `env.x` so the typing and defaults stay in one place.
 */
export const env = {
  /** Backend API base URL. */
  apiUrl: import.meta.env.VITE_API_URL ?? "http://localhost:8080",

  /**
   * Template used to build the workspace URL after tenant registration.
   * Must contain the literal `{slug}` placeholder.
   *
   * Examples:
   *   - dev:  http://{slug}.localhost:5173
   *   - prod: https://{slug}.nexus.com
   */
  workspaceUrlTemplate:
    import.meta.env.VITE_WORKSPACE_URL_TEMPLATE ??
    "http://{slug}.localhost:5173",
} as const

/** Builds the absolute workspace URL for a given tenant slug. */
export function workspaceUrl(slug: string, pathname: string = ""): string {
  const base = env.workspaceUrlTemplate.replace("{slug}", slug)
  if (!pathname) return base
  return `${base}${pathname.startsWith("/") ? pathname : `/${pathname}`}`
}
