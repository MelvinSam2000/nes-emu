FROM rust:1.61.0 AS builder
WORKDIR /usr/src/nes-web

# Install trunk
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

# Install binaryen for wasm-opt
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y binaryen

# Build wasm binary and set environment variables
RUN mkdir nes
COPY ./nes ./nes
RUN mkdir web
COPY ./web ./web
WORKDIR /usr/src/nes-web/web
RUN trunk build --release
RUN wasm-opt -Oz dist/nes-web_bg.wasm -o dist/nes-web_bg.wasm

# Deployment
FROM nginx:1.22.0-alpine

# Copy static files to nginx path
RUN mkdir /nes
ADD web/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /usr/src/nes-web/web/dist /nes

EXPOSE 8000/tcp