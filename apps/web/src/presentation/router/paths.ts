/**
 * Single source of truth for every URL in the app.
 *
 * - `segments` → relative URL parts, used in route definitions (`routes.tsx`)
 * - `paths`    → absolute URLs, used in <Link>, <Navigate>, navigate(), etc.
 *
 * Need to rename `/login` to `/sign-in`? Change one line below — every Link,
 * redirect, and route definition picks it up automatically.
 *
 * For dynamic routes, expose a function: `ticketDetail: (id) => ...`.
 */

const SEG = {
  login: "login",
  register: "register",
  app: "app",
  tickets: "tickets",
  knowledge: "knowledge",
  admin: "admin",
} as const

/** Relative segments — use these in route `path` definitions. */
export const segments = SEG

/** Absolute paths — use these in <Link>, navigate(), <Navigate>, etc. */
export const paths = {
  home: "/",

  login: `/${SEG.login}`,
  register: `/${SEG.register}`,

  app: {
    root: `/${SEG.app}`,
    tickets: `/${SEG.app}/${SEG.tickets}`,
    ticketDetail: (id: string) => `/${SEG.app}/${SEG.tickets}/${id}`,
    knowledge: `/${SEG.app}/${SEG.knowledge}`,
    admin: `/${SEG.app}/${SEG.admin}`,
  },
} as const

/** Type of all known path keys (useful for typing helpers). */
export type AppPaths = typeof paths
