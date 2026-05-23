import { z } from "zod"

export const loginSchema = z.object({
  email: z.email("Enter a valid email address"),
  password: z.string().min(1, "Password is required"),
})

/**
 * Tenant subdomain slug. Mirrors `validate_slug` in domain_identity (Rust):
 * - 3..=32 chars
 * - lowercase letters, digits, hyphens
 * - no leading/trailing hyphen, no consecutive hyphens
 *
 * The reserved-name check is still done server-side (single source of truth).
 */
export const tenantSlugSchema = z
  .string()
  .min(3, "Slug must be at least 3 characters")
  .max(32, "Slug must be at most 32 characters")
  .regex(
    /^[a-z0-9](?:[a-z0-9]|-(?!-))*[a-z0-9]$/,
    "Use lowercase letters, digits, and hyphens (no leading/trailing or consecutive hyphens)"
  )

export const registerSchema = z
  .object({
    tenantName: z.string().min(3, "Company name must be at least 3 characters"),
    tenantSlug: tenantSlugSchema,
    adminFullName: z.string().min(3, "Full name must be at least 3 characters"),
    adminEmail: z.email("Enter a valid email address"),
    adminPassword: z.string().min(8, "Password must be at least 8 characters"),
    confirmPassword: z.string(),
  })
  .refine((data) => data.adminPassword === data.confirmPassword, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  })

export type LoginInput = z.infer<typeof loginSchema>
export type RegisterInput = z.infer<typeof registerSchema>
