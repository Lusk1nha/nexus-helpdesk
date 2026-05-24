import { API } from "@nexus/api"
import type { CheckSlugResult } from "@nexus/auth"
import { tenantSlugSchema } from "@nexus/auth"
import { useQuery } from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

import { useDebouncedValue } from "./use-debounced-value"

/**
 * Real-time slug availability check.
 *
 * Returns React Query state (isPending / isError / data) for the *debounced*
 * slug. The query is automatically disabled while:
 *   - the slug is empty
 *   - the slug fails local Zod validation (saves a network round trip on
 *     obviously invalid input like "AB" or "foo--bar")
 *
 * The hook caller still owns the rendering decision (icon, message, color).
 */
export function useCheckSlugAvailability(slug: string) {
  const debouncedSlug = useDebouncedValue(slug.trim(), 300)
  const localCheck = tenantSlugSchema.safeParse(debouncedSlug)
  const enabled = localCheck.success && debouncedSlug.length > 0

  return useQuery({
    queryKey: ["check-slug", debouncedSlug],
    enabled,
    staleTime: 30_000,
    queryFn: () =>
      fetchApi<CheckSlugResult>(() =>
        http
          .get(API.identity.checkSlug, {
            searchParams: { slug: debouncedSlug },
          })
          .json()
      ),
  })
}
