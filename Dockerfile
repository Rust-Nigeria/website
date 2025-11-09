# # Get started with a build env with Rust nightly
# FROM rustlang/rust:nightly-alpine AS builder

# # Install dependencies
# RUN apk update && \
#     apk add --no-cache bash curl npm libc-dev binaryen git

# # Install SASS (used by stylance output) and Stylance
# RUN npm install -g sass

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
