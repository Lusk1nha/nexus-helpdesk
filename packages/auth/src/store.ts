import { create } from "zustand"
import { persist } from "zustand/middleware"

import type { LoginResult, User } from "./types"

interface AuthState {
  /** Short-lived access token, kept in memory. Lost on refresh — the silent
   * refresh flow rehydrates it from the httpOnly refresh cookie. */
  accessToken: string | null
  /** User identity, persisted so the app shell can render instantly on reload. */
  user: User | null

  setSession: (result: LoginResult) => void
  setAccessToken: (token: string) => void
  clear: () => void
  isAuthenticated: () => boolean
}

/**
 * Auth state store. The refresh token is NOT held in JS — it lives in an
 * httpOnly cookie. Only the user info is persisted to localStorage so we can
 * render `/app/...` immediately on page load (the access token is then
 * obtained via silent refresh on the first authenticated request).
 */
export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      accessToken: null,
      user: null,

      setSession: (result) =>
        set({
          accessToken: result.accessToken,
          user: {
            userId: result.userId,
            tenantId: result.tenantId,
            role: result.role,
          },
        }),

      setAccessToken: (token) => set({ accessToken: token }),

      clear: () => set({ accessToken: null, user: null }),

      isAuthenticated: () => get().user !== null,
    }),
    {
      name: "nexus:auth",
      partialize: (state) => ({ user: state.user }),
    }
  )
)
