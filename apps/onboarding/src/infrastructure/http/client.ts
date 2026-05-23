import { createApiClient, fetchApi as unwrapApi } from "@nexus/api"

export const http = createApiClient({
  baseUrl: import.meta.env.VITE_API_URL || "http://api.localhost:8080",
  // O onboarding não possui sessão ativa
  getAccessToken: () => null,
  setAccessToken: () => {},
  onAuthFailure: () => {},
})

// Re-exportamos para usar nos hooks
export const fetchApi = unwrapApi