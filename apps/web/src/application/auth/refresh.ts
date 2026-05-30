import { API } from "@nexus/api"
import { env } from "@/env"

/**
 * Single-flight refresh. The backend rotates refresh tokens on every use
 * (single-use), so two concurrent /refresh calls with the same cookie would
 * make the second fail with 401. We dedupe to a single in-flight request
 * shared by all callers (auth-init on load + the proactive scheduler).
 *
 * Resolves with the new access token; rejects on failure (cookie gone/expired).
 */
let inflight: Promise<string> | null = null

export function refreshAccessToken(): Promise<string> {
  if (inflight) return inflight

  inflight = fetch(`${env.apiUrl}/${API.identity.refresh}`, {
    method: "POST",
    credentials: "include",
  })
    .then((res) => (res.ok ? res.json() : Promise.reject(res.status)))
    .then((body: { data: { accessToken: string } }) => body.data.accessToken)
    .finally(() => {
      inflight = null
    })

  return inflight
}
