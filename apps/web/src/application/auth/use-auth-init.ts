import { useEffect, useState } from "react"
import { useAuthStore } from "@nexus/auth"

import { refreshAccessToken } from "./refresh"

/**
 * Proactively refreshes the access token on app load when there is a persisted
 * user but no in-memory access token (the normal state after a page reload).
 *
 * Returns `true` once the initialisation is complete (either the token was
 * refreshed successfully or there is no session to restore).
 */
export function useAuthInit(): boolean {
  const user = useAuthStore((s) => s.user)
  const accessToken = useAuthStore((s) => s.accessToken)
  const setAccessToken = useAuthStore((s) => s.setAccessToken)
  const clear = useAuthStore((s) => s.clear)

  const needsRefresh = user !== null && accessToken === null
  const [ready, setReady] = useState(!needsRefresh)

  useEffect(() => {
    if (!needsRefresh) {
      setReady(true)
      return
    }

    let cancelled = false

    refreshAccessToken()
      .then((token) => {
        if (!cancelled) setAccessToken(token)
      })
      .catch(() => {
        if (!cancelled) clear()
      })
      .finally(() => {
        if (!cancelled) setReady(true)
      })

    return () => {
      cancelled = true
    }
  }, [])

  return ready
}
