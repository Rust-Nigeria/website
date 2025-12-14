# syntax=docker/dockerfile:1.4

# # Chef stage for preparing recipe ----
# FROM rustlang/rust:nightly-alpine AS chef

# RUN apk update && \
#     apk add --no-cache musl-dev

# RUN cargo install cargo-chef

# WORKDIR /work


# ###### Planner stage ####
# FROM chef AS planner

# # Only copy files needed for cargo-chef to analyze dependencies
# COPY Cargo.toml Cargo.lock ./
# COPY src ./src
# RUN cargo chef prepare --recipe-path recipe.json


# ###### Builder stage ######
# FROM chef AS builder

# # Install system dependencies
# RUN apk update && \
#     apk add --no-cache bash curl npm libc-dev git

# # Install SASS (used by stylance output)
# RUN npm install -g sass

# # Install Rust tools
# RUN cargo install stylance-cli
# RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.0/cargo-leptos-installer.sh | sh

# RUN cargo install -f wasm-bindgen-cli --version 0.2.105
# # Add the WASM target
# RUN rustup target add wasm32-unknown-unknown


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



# Chef stage for preparing recipe ----
FROM rustlang/rust:nightly-alpine AS chef

RUN apk update && \
    apk add --no-cache musl-dev

RUN cargo install cargo-chef

WORKDIR /work


###### Planner stage ####
FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json


###### Builder stage ######
FROM chef AS builder

# Install system dependencies + sccache
RUN apk update && \
    apk add --no-cache bash curl npm libc-dev git ca-certificates openssl-dev openssl-libs-static pkgconfig

# Install SASS
RUN npm install -g sass

# Install sccache for compilation caching
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install sccache --locked

# Configure sccache environment variables
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
ENV SCCACHE_DIR=/sccache
ENV SCCACHE_CACHE_SIZE="10G"

# Install Rust tools with sccache
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/sccache \
    cargo install stylance-cli && \
    cargo install -f wasm-bindgen-cli --version 0.2.105

# Install cargo-leptos
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/sccache \
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.0/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work

# Copy recipe and cook dependencies with sccache
COPY --from=planner /work/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/sccache \
    cargo chef cook --release --recipe-path recipe.json

# Show sccache stats (optional, for debugging)
RUN sccache --show-stats

# Copy source code
COPY . .

# Run stylance
RUN stylance .

# Build with sccache caching
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/sccache \
    cargo leptos build --release -vv

# Show final sccache stats
RUN sccache --show-stats


##### Production runner #####
FROM rustlang/rust:nightly-alpine AS runner

WORKDIR /app

COPY --from=builder /work/data /app/data
COPY --from=builder /work/target/release/rust-nigeria-website /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="debug"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/rust-nigeria-website"]