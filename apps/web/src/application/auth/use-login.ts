import { useMutation } from "@tanstack/react-query"

import type { LoginInput } from "@/domain/auth/auth.schemas"
import type { LoginResult } from "@/domain/auth/auth.types"
import { fetchApi, http } from "@/infrastructure/http/client"
import { useAuthStore } from "@/infrastructure/store/auth.store"
import { API } from "@/infrastructure/http/api.routes"

export function useLogin() {
  const setSession = useAuthStore((s) => s.setSession)

  return useMutation({
    mutationFn: (input: LoginInput) =>
      fetchApi<LoginResult>(() =>
        http.post(API.identity.login, { json: input }).json()
      ),
    onSuccess: (result) => {
      setSession(result)
    },
  })
}
