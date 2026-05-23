import { API } from "@nexus/api"
import type { RegisterInput, RegisterResult } from "@nexus/auth"
import { useMutation } from "@tanstack/react-query"

import { fetchApi, http } from "@/infrastructure/http/client"

export function useRegister() {
  return useMutation({
    mutationFn: ({ confirmPassword: _, ...input }: RegisterInput) =>
      fetchApi<RegisterResult>(() =>
        http.post(API.identity.register, { json: input }).json()
      ),
  })
}
