# Knot Space

Web server for Knot packages built with [Hono](https://hono.dev), Prisma and Zod.

## Development

```bash
npm install
npm run dev
```

The server listens on `http://localhost:8000` by default.

### Environment

The server requires the following variables:

- `DATABASE_URL` – PostgreSQL connection string
- `JWT_SECRET` – secret used to sign authentication tokens
- `PORT` – optional port (defaults to `8000`)

You can provide them via a `.env` file or the shell.

## Production

```bash
npm install
npm run build
JWT_SECRET=... DATABASE_URL=... npm start
```

Docker users can build and run the container:

```bash
docker build -t knot-space .
docker run -e DATABASE_URL=... -e JWT_SECRET=... -p 8000:8000 knot-space
```

## API

- `POST /signup` - register a new user
- `POST /login` - obtain a JWT token
- `GET /me` - get current user info (requires `Authorization: Bearer <token>` header)
