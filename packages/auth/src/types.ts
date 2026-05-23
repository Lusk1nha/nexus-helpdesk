export type Role = "admin" | "agent" | "customer"

export interface User {
  userId: string
  tenantId: string
  role: Role
}

/**
 * Login response — the refresh token is NOT here. It lives in an httpOnly
 * cookie set by the backend and sent automatically by the browser.
 */
export interface LoginResult {
  accessToken: string
  accessTokenExpiresIn: number
  userId: string
  tenantId: string
  role: Role
}

export interface RegisterResult {
  tenantId: string
  tenantSlug: string
  userId: string
  message: string
}

export interface CheckSlugResult {
  slug: string
  available: boolean
  reason: string | null
}
