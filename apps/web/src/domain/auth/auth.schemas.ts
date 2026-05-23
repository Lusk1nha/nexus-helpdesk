import { z } from 'zod'

export const loginSchema = z.object({
  email: z.email('Enter a valid email address'),
  password: z.string().min(1, 'Password is required'),
})

export const registerSchema = z
  .object({
    tenantName: z.string().min(3, 'Company name must be at least 3 characters'),
    adminFullName: z.string().min(3, 'Full name must be at least 3 characters'),
    adminEmail: z.email('Enter a valid email address'),
    adminPassword: z.string().min(8, 'Password must be at least 8 characters'),
    confirmPassword: z.string(),
  })
  .refine((data) => data.adminPassword === data.confirmPassword, {
    message: 'Passwords do not match',
    path: ['confirmPassword'],
  })

export type LoginInput = z.infer<typeof loginSchema>
export type RegisterInput = z.infer<typeof registerSchema>
