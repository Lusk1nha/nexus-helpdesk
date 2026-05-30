import { API } from "@nexus/api"
import {
  useAuthStore,
  type CustomerSignupInput,
  type LoginResult,
} from "@nexus/auth"
import { useMutation } from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

/**
 * Customer self-signup. Creates a `customer` account in the current tenant
 * (resolved from the subdomain `slug`) and logs the user in on success —
 * the backend returns the same shape as `/login`.
 */
export function useSignup(slug: string | null) {
  const setSession = useAuthStore((s) => s.setSession)

  return useMutation({
    mutationFn: (input: CustomerSignupInput) =>
      fetchApi<LoginResult>(() =>
        http
          .post(API.identity.signup, {
            json: {
              slug,
              fullName: input.fullName,
              email: input.email,
              password: input.password,
            },
          })
          .json()
      ),
    onSuccess: (result) => {
      setSession(result)
    },
  })
}
