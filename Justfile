setup:
    cargo install cargo-leptos
    cargo install stylance-cli

start:
    stylance . --watch &
    cargo leptos watch

build:
    stylance . &&
    cargo leptos build

clean:
    cargo clean

fmt:
    cargo fmt

test:
    cargo test
