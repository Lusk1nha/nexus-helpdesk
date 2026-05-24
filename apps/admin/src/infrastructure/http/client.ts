import { createApiClient, fetchApi } from "@nexus/api"
import { useAuthStore } from "@nexus/auth"

import { env } from "@/env"
import { paths } from "@/presentation/router/paths"

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
