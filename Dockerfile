FROM rust:1.92.0-slim-bookworm AS base

RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    curl \
    npm    

COPY rust-toolchain.toml ./
RUN rustup show
# Install all Rust tools once in base
RUN cargo install cargo-binstall
RUN cargo install cargo-chef 
RUN npm install -g sass
RUN cargo install stylance-cli 
RUN cargo binstall cargo-leptos -y
RUN cargo install -f wasm-bindgen-cli --version 0.2.105
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work

###### Planner stage ####
FROM base AS planner

# Only copy dependency files, NOT source code
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

###### Chef stage - cook dependencies ####
FROM base AS chef

COPY --from=planner /work/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

###### Builder stage ######
FROM base AS builder

WORKDIR /work

# Copy cooked dependencies
COPY --from=chef /work/target target
COPY --from=chef /usr/local/cargo /usr/local/cargo

# Now copy source code
COPY . .

# Run stylance and build
RUN stylance .
RUN cargo leptos build --release -vv

##### Production runner #####
FROM debian:bookworm-slim AS runner

WORKDIR /app

# Install runtime dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy only what's needed for runtime
COPY --from=builder /work/data /app/data
COPY --from=builder /work/target/release/rust-nigeria-website /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/




ENV RUST_LOG="debug"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site

EXPOSE 8080

CMD ["/app/rust-nigeria-website"]



# # Chef stage for preparing recipe ----
# FROM rustlang/rust:nightly-alpine AS chef

# RUN apk update && \
#     apk add --no-cache musl-dev bash curl npm libc-dev git

# COPY rust-toolchain.toml ./
# RUN rustup show
# # Install all Rust tools once in base
# RUN cargo install cargo-chef --locked
# RUN npm install -g sass
# RUN cargo install stylance-cli
# RUN cargo install cargo-leptos --version 0.3.0 --locked
# RUN cargo install -f wasm-bindgen-cli --version 0.2.105
# # RUN rustup target add wasm32-unknown-unknown

# WORKDIR /work


# ###### Planner stage ####
# FROM chef AS planner

# # Only copy files needed for cargo-chef to analyze dependencies
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json


# ###### Builder stage ######
# FROM chef AS builder

# WORKDIR /work

# # Copy recipe and cook dependencies (this layer gets cached)
# COPY --from=planner /work/recipe.json recipe.json
# RUN cargo chef cook --release --recipe-path recipe.json

# # Now copy the actual source code
# COPY . .

# # Run stylance first
# RUN stylance .

# RUN cargo leptos build --release -vv


# ##### Production runner #####
# FROM rustlang/rust:nightly-alpine AS runner

# WORKDIR /app

# COPY --from=builder /work/data /app/data
# COPY --from=builder /work/target/release/rust-nigeria-website /app/
# COPY --from=builder /work/target/site /app/site
# COPY --from=builder /work/Cargo.toml /app/

# ENV RUST_LOG="debug"
# ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
# ENV LEPTOS_SITE_ROOT=./site
# EXPOSE 8080

# CMD ["/app/rust-nigeria-website"]