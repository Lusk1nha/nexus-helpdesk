import { useMutation } from "@tanstack/react-query"
import type { RegisterInput, RegisterResult } from "@nexus/auth"

import { fetchApi, http } from "@/infrastructure/http/client"
import { API } from "@nexus/api"

export function useRegister() {
  return useMutation({
    mutationFn: (input: RegisterInput) =>
      fetchApi<RegisterResult>(() =>
        http.post(API.identity.register, { json: input }).json()
      ),
  })
}