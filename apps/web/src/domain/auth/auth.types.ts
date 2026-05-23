export type Role = "admin" | "agent" | "customer"

export interface User {
  userId: string
  tenantId: string
  role: Role
}

export interface TokenPair {
  accessToken: string
  refreshToken: string
  accessTokenExpiresIn: number
}

export interface LoginResult extends TokenPair {
  userId: string
  tenantId: string
  role: Role
}

export interface RegisterResult {
  tenantId: string
  userId: string
  message: string
}
