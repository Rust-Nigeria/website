###### Base stage — image toolchain (1.97.1) builds the tools ######
FROM rust:1.97.1-alpine3.23 AS base

RUN apk add --no-cache \
    bash \
    curl \
    git \
    npm \
    build-base \
    openssl-dev \
    pkgconf \
    libc-dev \
    musl-dev \
    binaryen \
    perl \
    python3 \
    cmake

# IMPORTANT: install tooling BEFORE rust-toolchain.toml is copied in.
# Otherwise the pinned nightly overrides the image toolchain and tool
# dependencies (kstring, vergen, ...) fail their rustc version checks.
RUN cargo install --locked cargo-binstall
RUN cargo binstall -y --locked cargo-chef stylance-cli cargo-leptos
RUN cargo binstall -y wasm-bindgen-cli --version 0.2.126
RUN npm install -g sass

# Now pin the project toolchain and add the wasm target to *it*
COPY rust-toolchain.toml ./
RUN rustup show \
    && rustup target add wasm32-unknown-unknown

WORKDIR /work

###### Planner stage ######
FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

###### Chef stage — cook dependencies ######
FROM base AS chef
COPY --from=planner /work/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

###### Builder stage ######
FROM base AS builder
WORKDIR /work

# Reuse cooked dependency artifacts
COPY --from=chef /work/target target
COPY --from=chef /usr/local/cargo /usr/local/cargo

COPY . .

RUN stylance .
RUN cargo leptos build --release -vv

###### Production runner ######
FROM debian:bookworm-slim AS runner
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /work/data /app/data
COPY --from=builder /work/target/release/rust-nigeria-website /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="debug"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site

EXPOSE 8080
CMD ["/app/rust-nigeria-website"]
