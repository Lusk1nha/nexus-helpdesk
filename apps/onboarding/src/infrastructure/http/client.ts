import { createApiClient, fetchApi as unwrapApi } from "@nexus/api"

import { env } from "@/env"

/**
 * Onboarding is a public app — no session, no token storage. The factory
 * still gets the hooks so the contract matches `apps/web` and any future
 * authenticated flows can drop in.
 */
export const http = createApiClient({
  baseUrl: env.apiUrl,
  getAccessToken: () => null,
  setAccessToken: () => {},
  onAuthFailure: () => {},
})

export const fetchApi = unwrapApi
