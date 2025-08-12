FROM oven/bun AS builder
WORKDIR /app

RUN apt-get update -y && apt-get install -y openssl

COPY package.json bun.lock ./
RUN bun install


ARG NODE_ENV
ENV NODE_ENV=$NODE_ENV

COPY . .
RUN bun run build


EXPOSE 8080

CMD ["sh", "-c", "./server"]
