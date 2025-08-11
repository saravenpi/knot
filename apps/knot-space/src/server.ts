import { Hono } from 'hono'
import { serve } from '@hono/node-server'
import { z } from 'zod'
import { zValidator } from '@hono/zod-validator'
import { PrismaClient, Prisma } from '@prisma/client'
import { sign, verify } from 'jsonwebtoken'
import { hash, compare } from 'bcryptjs'

const env = z
  .object({
    DATABASE_URL: z.string().url(),
    JWT_SECRET: z.string().min(1),
    PORT: z.coerce.number().default(8000)
  })
  .parse(process.env)

const prisma = new PrismaClient({
  datasources: {
    db: { url: env.DATABASE_URL }
  }
})
const app = new Hono<{ Variables: { userId: number } }>()

const auth = async (c: any, next: any) => {
  const header = c.req.header('authorization')
  if (!header) return c.json({ error: 'Unauthorized' }, 401)
  try {
    const token = header.split(' ')[1]
    const payload = verify(token, env.JWT_SECRET) as { userId: number }
    c.set('userId', payload.userId)
    await next()
  } catch {
    return c.json({ error: 'Unauthorized' }, 401)
  }
}

app.get('/', c => c.text('Knot Space API'))

app.post(
  '/signup',
  zValidator(
    'json',
    z.object({
      email: z.string().email(),
      password: z.string().min(8)
    })
  ),
  async c => {
    const { email, password } = c.req.valid('json')
    const hashed = await hash(password, 10)
    try {
      const user = await prisma.user.create({ data: { email, password: hashed } })
      return c.json({ id: user.id, email: user.email }, 201)
    } catch (err) {
      if (
        err instanceof Prisma.PrismaClientKnownRequestError &&
        err.code === 'P2002'
      ) {
        return c.json({ error: 'Email already registered' }, 409)
      }
      throw err
    }
  }
)

app.post('/login', zValidator('json', z.object({
  email: z.string().email(),
  password: z.string().min(8)
})), async c => {
  const { email, password } = c.req.valid('json')
  const user = await prisma.user.findUnique({ where: { email } })
  if (!user || !(await compare(password, user.password))) {
    return c.json({ error: 'Invalid credentials' }, 401)
  }
  const token = sign({ userId: user.id }, env.JWT_SECRET, { expiresIn: '1h' })
  return c.json({ token })
})

app.get('/me', auth, async c => {
  const userId = c.get('userId')
  const user = await prisma.user.findUnique({ where: { id: userId }, select: { id: true, email: true } })
  return c.json(user)
})

const port = env.PORT
serve({ fetch: app.fetch, port })
console.log(`🚀 Knot Space running on http://localhost:${port}`)

process.on('SIGINT', async () => {
  await prisma.$disconnect()
  process.exit(0)
})
