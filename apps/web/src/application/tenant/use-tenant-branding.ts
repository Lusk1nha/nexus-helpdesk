import { API } from "@nexus/api"
import { type ThemeId } from "@nexus/theme"
import { useQuery } from "@tanstack/react-query"

import { env } from "@/env"

interface TenantBranding {
  slug: string
  name: string
  theme: ThemeId
}

async function fetchBranding(slug: string): Promise<TenantBranding> {
  const url = `${env.apiUrl}/${API.identity.tenantBranding}?slug=${encodeURIComponent(slug)}`
  const res = await fetch(url)
  if (!res.ok) throw new Error("branding fetch failed")
  const body = await res.json()
  return body.data as TenantBranding
}

export function useTenantBranding(slug: string | null) {
  return useQuery({
    queryKey: ["tenant-branding", slug],
    queryFn: () => fetchBranding(slug!),
    enabled: slug != null,
    staleTime: 5 * 60 * 1000,
  })
}
