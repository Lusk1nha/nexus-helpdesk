import ky, { type AfterResponseState, type BeforeRequestState, type KyInstance } from 'ky'

import { env } from '@/env'
import { useAuthStore } from '@/infrastructure/store/auth.store'
import { API } from './api.routes'

/**
 * Shared ky instance with:
 * - Auth header injection (beforeRequest)
 * - Transparent access-token refresh on 401 (afterResponse)
 */
export const http: KyInstance = ky.create({
  baseUrl: env.apiUrl,
  timeout: 30_000,
  retry: 0,
  hooks: {
    beforeRequest: [
      ({ request }: BeforeRequestState) => {
        const token = useAuthStore.getState().accessToken
        if (token) {
          request.headers.set('Authorization', `Bearer ${token}`)
        }
      },
    ],
    afterResponse: [
      async ({ request, response }: AfterResponseState): Promise<Response | void> => {
        if (response.status !== 401) return

        const { refreshToken, setAccessToken, clear } = useAuthStore.getState()

        if (!refreshToken) {
          clear()
          window.location.href = '/login'
          return
        }

        try {
          const refreshed = await ky
            .post(`${env.apiUrl}/${API.identity.refresh}`, {
              json: { refreshToken },
            })
            .json<{ data: { accessToken: string } }>()

          setAccessToken(refreshed.data.accessToken)

          return ky(request.url, {
            method: request.method,
            headers: {
              ...Object.fromEntries(request.headers.entries()),
              Authorization: `Bearer ${refreshed.data.accessToken}`,
            },
          })
        } catch {
          clear()
          window.location.href = '/login'
        }
      },
    ],
  },
})

/** Unwrap the ApiResponse envelope and return `data`. */
export async function fetchApi<T>(fn: () => Promise<{ data: T }>): Promise<T> {
  const envelope = await fn()
  return envelope.data
}
