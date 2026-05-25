export const env = {
  apiUrl: import.meta.env.VITE_API_URL ?? "http://localhost:8080",
  onboardingUrl:
    import.meta.env.VITE_ONBOARDING_URL ?? "http://onboarding.nexus.localhost",
} as const
