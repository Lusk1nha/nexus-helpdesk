import { useEffect } from "react"
import { useAuthStore } from "@nexus/auth"

import { refreshAccessToken } from "./refresh"

/** Refresh this many ms before the access token actually expires. */
const REFRESH_BUFFER_MS = 60_000

/** Decode the `exp` claim (unix seconds) from a JWT. Null if unparseable. */
function getExp(token: string): number | null {
  try {
    const base64url = token.split(".")[1]
    const base64 = base64url?.replace(/-/g, "+").replace(/_/g, "/")
    const padded = base64?.padEnd(
      base64.length + ((4 - (base64.length % 4)) % 4),
      "="
    )

    if (!padded) return null

    const payload = JSON.parse(atob(padded))
    return typeof payload.exp === "number" ? payload.exp : null
  } catch {
    return null
  }
}

/**
 * Proactively rotates the access token shortly before it expires. When the
 * token in the store changes, dependents (the `ky` client + the SSE
 * EventSource, which reads the token from the store) pick up the new value —
 * the EventSource reconnects automatically with a fresh token.
 *
 * Each successful refresh re-runs this effect (accessToken is a dependency),
 * scheduling the next refresh — a self-perpetuating chain while logged in.
 */
export function useTokenRefreshScheduler() {
  const accessToken = useAuthStore((s) => s.accessToken)
  const setAccessToken = useAuthStore((s) => s.setAccessToken)
  const clear = useAuthStore((s) => s.clear)

  useEffect(() => {
    if (!accessToken) return

    const exp = getExp(accessToken)
    if (!exp) return

    const delay = Math.max(0, exp * 1000 - Date.now() - REFRESH_BUFFER_MS)

    const id = setTimeout(() => {
      refreshAccessToken()
        .then(setAccessToken)
        .catch(() => clear())
    }, delay)

    return () => clearTimeout(id)
  }, [accessToken, setAccessToken, clear])
}
