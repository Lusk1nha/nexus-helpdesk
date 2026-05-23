import { createApiClient, fetchApi } from "@nexus/api"
import { useAuthStore } from "@nexus/auth"

import { env } from "@/env"
import { paths } from "@/presentation/router/paths"

/**
 * App-local ky instance wired to this app's Zustand auth store + redirect path.
 * Each app (web, onboarding, admin) creates its own instance, but they all
 * share the same factory + the same cookie-based refresh contract.
 */
export const http = createApiClient({
  baseUrl: env.apiUrl,
  getAccessToken: () => useAuthStore.getState().accessToken,
  setAccessToken: (token) => useAuthStore.getState().setAccessToken(token),
  onAuthFailure: () => {
    useAuthStore.getState().clear()
    window.location.href = paths.login
  },
})

export { fetchApi }
