import ky, {
  type AfterResponseState,
  type BeforeRequestState,
  type KyInstance,
} from "ky"

import { API } from "./routes"

/**
 * Caller-supplied hooks. Apps inject:
 *   - getAccessToken()  → current in-memory JWT (Bearer header)
 *   - setAccessToken()  → store the rotated token after a silent refresh
 *   - onAuthFailure()   → redirect to login when refresh fails (cookie gone/expired)
 *
 * The refresh token itself lives in an httpOnly cookie that the browser sends
 * automatically thanks to `credentials: "include"` — no JavaScript handling.
 */
export interface ApiClientOptions {
  baseUrl: string
  getAccessToken: () => string | null
  setAccessToken: (token: string) => void
  onAuthFailure: () => void
}

/**
 * Builds the shared ky instance for a frontend app. Each app (web, onboarding,
 * admin) creates its own instance with its own auth hooks.
 */
export function createApiClient(opts: ApiClientOptions): KyInstance {
  return ky.create({
    baseUrl: opts.baseUrl,
    credentials: "include", // critical for cookie-based refresh
    timeout: 30_000,
    retry: 0,
    hooks: {
      beforeRequest: [
        ({ request }: BeforeRequestState) => {
          const token = opts.getAccessToken()
          if (token) {
            request.headers.set("Authorization", `Bearer ${token}`)
          }
        },
      ],
      afterResponse: [
        async ({
          request,
          response,
        }: AfterResponseState): Promise<Response | void> => {
          if (response.status !== 401) return

          // Silent refresh — the cookie is sent automatically.
          try {
            const refreshed = await ky
              .post(`${opts.baseUrl}/${API.identity.refresh}`, {
                credentials: "include",
              })
              .json<{ data: { accessToken: string } }>()

            opts.setAccessToken(refreshed.data.accessToken)

            return ky(request.url, {
              method: request.method,
              credentials: "include",
              headers: {
                ...Object.fromEntries(request.headers.entries()),
                Authorization: `Bearer ${refreshed.data.accessToken}`,
              },
            })
          } catch {
            opts.onAuthFailure()
          }
        },
      ],
    },
  })
}

/** Unwrap the `{ data, meta }` envelope used by every backend response. */
export async function fetchApi<T>(fn: () => Promise<{ data: T }>): Promise<T> {
  const envelope = await fn()
  return envelope.data
}
