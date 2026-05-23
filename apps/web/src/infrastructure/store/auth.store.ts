import { create } from 'zustand'
import { persist } from 'zustand/middleware'

import type { LoginResult, User } from '@/domain/auth/auth.types'

interface AuthState {
  accessToken: string | null
  refreshToken: string | null
  user: User | null

  setSession: (result: LoginResult) => void
  setAccessToken: (token: string) => void
  clear: () => void
  isAuthenticated: () => boolean
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      accessToken: null,
      refreshToken: null,
      user: null,

      setSession: (result) =>
        set({
          accessToken: result.accessToken,
          refreshToken: result.refreshToken,
          user: {
            userId: result.userId,
            tenantId: result.tenantId,
            role: result.role,
          },
        }),

      setAccessToken: (token) => set({ accessToken: token }),

      clear: () => set({ accessToken: null, refreshToken: null, user: null }),

      isAuthenticated: () => get().accessToken !== null && get().user !== null,
    }),
    {
      name: 'nexus:auth',
      // Only persist the refresh token and user — access token is ephemeral
      partialize: (state) => ({
        refreshToken: state.refreshToken,
        user: state.user,
      }),
    },
  ),
)
