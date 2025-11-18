# # Get started with a build env with Rust nightly
# FROM rustlang/rust:nightly-alpine AS builder

# # Install dependencies
# RUN apk update && \
#     apk add --no-cache bash curl npm libc-dev binaryen git openssl-dev perl make musl-dev

# # RUN curl -L https://github.com/tailwindlabs/tailwindcss/releases/download/v4.1.10/tailwindcss-linux-x64-musl -o /usr/local/bin/tailwindcss && \
# #     chmod +x /usr/local/bin/tailwindcss && \
# #     tailwindcss --version
# # Install SASS (used by stylance output) and Stylance
# # RUN npm install -g sass
# RUN npm install -g sass @tailwindcss/cli
# # RUN npm install -g sass


# # Install stylance-cli from crates.io
# RUN cargo install stylance-cli

# # Install cargo-leptos
# # RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
# RUN cargo install cargo-leptos
# # Add the WASM target
# RUN rustup target add wasm32-unknown-unknown

# # Set up workspace
# WORKDIR /work
# COPY . .

# # ✅ Run stylance first
# RUN stylance .

# # ✅ Then run the leptos build
# RUN cargo leptos build --release -vv

# # ---- Production runner ----
# FROM rustlang/rust:nightly-alpine AS runner

# WORKDIR /app

# COPY --from=builder /work/data /app/data
# COPY --from=builder /work/target/release/rust-nigeria-website /app/
# COPY --from=builder /work/target/site /app/site
# COPY --from=builder /work/Cargo.toml /app/

# ENV RUST_LOG="info"
# ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
# ENV LEPTOS_SITE_ROOT=./site
# EXPOSE 8080

# CMD ["/app/rust-nigeria-website"]


# ---- Builder stage ----
FROM rustlang/rust:nightly-alpine AS builder

# Install dependencies
RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen git openssl-dev perl make musl-dev

# Install SASS and Tailwind CSS
RUN npm install -g sass @tailwindcss/cli

# Install Stylance CLI
RUN cargo install stylance-cli

# Install cargo-leptos
RUN cargo install cargo-leptos

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Set up workspace
WORKDIR /work
COPY . .

# Run stylance to generate CSS modules
RUN stylance .

# Build the Leptos app
RUN cargo leptos build --release -vv

# ---- Production runner ----
FROM rustlang/rust:nightly-alpine AS runner

WORKDIR /app

# Copy built artifacts
COPY --from=builder /work/data /app/data
COPY --from=builder /work/target/release/rust-nigeria-website /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site

EXPOSE 8080

CMD ["/app/rust-nigeria-website"]