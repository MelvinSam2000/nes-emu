# Builder
FROM docker.io/rust:1.65.0 AS builder

ARG NES_PATH="/usr/src/nes-web"
WORKDIR ${NES_PATH}

# Install trunk
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

# Install binaryen for wasm-opt
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y binaryen

# Build project
RUN mkdir nes
COPY ./nes ./nes
RUN mkdir web
COPY ./web ./web
WORKDIR ${NES_PATH}/web
RUN trunk build --release
RUN wasm-opt -O3 dist/nes-web_bg.wasm -o dist/nes-web_bg.wasm

# Deployment
FROM docker.io/nginx:1.23.0-alpine

# Copy static files to nginx path
RUN mkdir /nes
ADD web/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /usr/src/nes-web/web/dist /nes

EXPOSE 8000/tcp