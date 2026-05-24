/**
 * Single source of truth for every URL in the onboarding app.
 *
 * - `segments` → relative URL parts, used in route definitions (`routes.tsx`)
 * - `paths`    → absolute URLs, used in <Link>, <Navigate>, navigate(), etc.
 */

const SEG = {
  home: "",
  register: "register",
  about: "about",
} as const

/** Relative segments — use these in route `path` definitions. */
export const segments = SEG

/** Absolute paths — use these in <Link>, navigate(), <Navigate>, etc. */
export const paths = {
  home: "/",
  register: `/${SEG.register}`,
  about: `/${SEG.about}`,
} as const

export type AppPaths = typeof paths
