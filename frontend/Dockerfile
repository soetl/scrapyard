FROM node:18-alpine

RUN mkdir -p user/src/frontend

WORKDIR /usr/src/frontend

COPY . .

RUN npm ci && npm cache clean --force
RUN npm run build

EXPOSE 3000