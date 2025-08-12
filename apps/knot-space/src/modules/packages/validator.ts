import { zValidator } from '@hono/zod-validator';
import { PublishPackageSchema } from '@/types';
import { z } from 'zod';

export const validatePublishPackage = zValidator('json', PublishPackageSchema);

export const validatePackageQuery = zValidator('query', z.object({
  search: z.string().optional(),
  owner: z.string().optional(),
  team: z.string().optional(),
  tags: z.string().optional(),
  limit: z.string().transform(val => parseInt(val) || 20).optional(),
  offset: z.string().transform(val => parseInt(val) || 0).optional(),
}));