import { z } from 'zod';

// Re-export shared types
export * from '#/types';

// Keep the zod schemas for validation
export const CreateUserSchema = z.object({
  username: z
    .string()
    .min(3, 'Username must be at least 3 characters')
    .max(50, 'Username must be at most 50 characters')
    .regex(
      /^[a-zA-Z0-9][a-zA-Z0-9\-_]*$/,
      'Username must start with alphanumeric and contain only alphanumeric, hyphens, or underscores'
    )
    .transform((val) => val.trim().toLowerCase()),
  email: z
    .string()
    .email('Invalid email format')
    .max(255, 'Email must be at most 255 characters')
    .transform((val) => val.trim().toLowerCase()),
  password: z
    .string()
    .min(6, 'Password must be at least 6 characters')
    .max(128, 'Password must be at most 128 characters')
    .regex(
      /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/,
      'Password must contain at least one lowercase, uppercase letter and number'
    ),
});

export const LoginSchema = z.object({
  username: z
    .string()
    .min(1, 'Username is required')
    .max(50, 'Username too long')
    .transform((val) => val.trim().toLowerCase()),
  password: z.string().min(1, 'Password is required').max(128, 'Password too long'),
});

export const CreateTeamSchema = z.object({
  name: z
    .string()
    .min(3, 'Team name must be at least 3 characters')
    .max(50, 'Team name must be at most 50 characters')
    .regex(/^[a-z0-9][a-z0-9-]*$/, 'Team name must be lowercase alphanumeric with hyphens')
    .transform((val) => val.trim().toLowerCase()),
  description: z
    .string()
    .max(500, 'Description must be at most 500 characters')
    .optional()
    .transform((val) => val?.trim()),
});

export const AddTeamMemberSchema = z.object({
  username: z
    .string()
    .min(1, 'Username is required')
    .max(50, 'Username too long')
    .transform((val) => val.trim().toLowerCase()),
  role: z.enum(['owner', 'admin', 'member'], {
    errorMap: () => ({ message: 'Role must be owner, admin, or member' }),
  }),
});

export const PublishPackageSchema = z.object({
  name: z
    .string()
    .min(1, 'Package name is required')
    .max(100, 'Package name must be at most 100 characters')
    .regex(/^(@[a-z0-9-]+\/)?[a-z0-9][a-z0-9-]*$/, 'Invalid package name format')
    .transform((val) => val.trim().toLowerCase()),
  version: z
    .string()
    .min(1, 'Version is required')
    .max(20, 'Version must be at most 20 characters')
    .regex(
      /^[0-9]+\.[0-9]+\.[0-9]+(-[a-z0-9-]+)?(\+[a-z0-9-]+)?$/,
      'Version must follow semantic versioning'
    ),
  description: z
    .string()
    .max(1000, 'Description must be at most 1000 characters')
    .optional()
    .transform((val) => val?.trim()),
  teamName: z
    .union([
      z.string()
        .max(50, 'Team name too long')
        .regex(/^[a-z0-9][a-z0-9-]*$/, 'Invalid team name format')
        .transform((val) => val.trim().toLowerCase()),
      z.null(),
      z.undefined()
    ])
    .optional(),
  tags: z
    .array(
      z
        .string()
        .min(1, 'Tag cannot be empty')
        .max(30, 'Tag must be at most 30 characters')
        .regex(/^[a-z0-9][a-z0-9-]*$/, 'Tag must be lowercase alphanumeric with hyphens')
        .transform((val) => val.trim().toLowerCase())
    )
    .max(10, 'Maximum 10 tags allowed')
    .optional(),
});

export const UpdateProfileSchema = z.object({
  username: z
    .string()
    .min(3, 'Username must be at least 3 characters')
    .max(50, 'Username must be at most 50 characters')
    .regex(
      /^[a-zA-Z0-9][a-zA-Z0-9\-_]*$/,
      'Username must start with alphanumeric and contain only alphanumeric, hyphens, or underscores'
    )
    .transform((val) => val.trim().toLowerCase()),
  email: z
    .string()
    .email('Invalid email format')
    .max(255, 'Email must be at most 255 characters')
    .transform((val) => val.trim().toLowerCase()),
});

export type CreateUserRequest = z.infer<typeof CreateUserSchema>;
export type LoginRequest = z.infer<typeof LoginSchema>;
export type CreateTeamRequest = z.infer<typeof CreateTeamSchema>;
export type AddTeamMemberRequest = z.infer<typeof AddTeamMemberSchema>;
export type PublishPackageRequest = z.infer<typeof PublishPackageSchema>;
export type UpdateProfileRequest = z.infer<typeof UpdateProfileSchema>;
