FROM node:20-slim
WORKDIR /app

ENV NODE_ENV=production

COPY package.json package-lock.json tsconfig.app.json ./
RUN npm ci --omit=dev

COPY prisma ./prisma
RUN npx prisma generate

COPY src ./src
RUN npm run build

COPY start.sh ./start.sh
RUN chmod +x start.sh

EXPOSE 8000
CMD ["./start.sh"]
