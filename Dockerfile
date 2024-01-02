FROM rust:1-slim-bullseye AS server-builder
WORKDIR /server

RUN USER=root apt update -y && apt install pkg-config libssl-dev -y

COPY ./server/Cargo.toml .
RUN cargo fetch

COPY ./server/src ./src
RUN cargo build --release

FROM node:20-alpine AS client-builder
WORKDIR /client

COPY ./client .

RUN npm i -g pnpm && pnpm i 

RUN pnpm build

FROM debian:bullseye-slim
WORKDIR /app

COPY --from=server-builder /server/target/release/server ./

COPY --from=client-builder /client/dist ./static/dist
COPY --from=client-builder /client/.env ./

EXPOSE 8080
CMD [ "/app/server" ]