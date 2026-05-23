import { API } from "@nexus/api"
import { useAuthStore, type LoginInput, type LoginResult } from "@nexus/auth"
import { useMutation } from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

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
