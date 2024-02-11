# We must be able to control the name of the application build!
# By letting it contain a random part, cache-busting is made possible.
# Overwrite this using a `--build-arg`, like in:
# `docker build --build-arg DEFAULT_LEPTOS_OUTPUT_NAME="book-ssr-$(date +%s)" -t image-name .`
ARG DEFAULT_LEPTOS_OUTPUT_NAME="book-ssr"

# ------------------------------------------------------------------------------
# Build
# ------------------------------------------------------------------------------

FROM --platform=linux/amd64 rust:1.75-slim-bookworm as builder

WORKDIR /build

ARG DEFAULT_LEPTOS_OUTPUT_NAME

RUN <<EOF
    apt update
    apt upgrade
    apt install -y curl tar ca-certificates pkg-config libssl-dev git unzip gzip brotli zstd musl-tools musl-dev clang llvm npm --no-install-recommends

    rustup default stable
    rustup target add x86_64-unknown-linux-musl
    rustup target add wasm32-unknown-unknown

    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

    cargo binstall cargo-leptos -y

    # Cleanup
    rm -rf ${CARGO_HOME}/git/*
    rm -rf ${CARGO_HOME}/registry/*
    rm -rf /var/lib/apt/lists/*
EOF

COPY . /build

# Configuring ENV variables to alter the `cargo leptos build` command.
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-musl"
ENV LEPTOS_OUTPUT_NAME=${DEFAULT_LEPTOS_OUTPUT_NAME}

RUN <<EOF
    cargo leptos build --release

    /bin/bash ./precompress.sh "/build/target/site"
EOF

# ------------------------------------------------------------------------------
# Final
# ------------------------------------------------------------------------------

FROM alpine:latest

WORKDIR /app

ARG DEFAULT_LEPTOS_OUTPUT_NAME

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/book-ssr .
COPY --from=builder /build/target/site ./site
COPY --from=builder /build/certs/ssl_cert.pem ./certs/www.leptonic.dev-cert.pem
COPY --from=builder /build/certs/ssl_key.pem ./certs/www.leptonic.dev-key.pem

ENV TLS_CERT_PATH="certs/www.leptonic.dev-cert.pem"
ENV TLS_KEY_PATH="certs/www.leptonic.dev-key.pem"
ENV LEPTOS_OUTPUT_NAME=${DEFAULT_LEPTOS_OUTPUT_NAME}
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:443"

CMD ["/app/book-ssr"]
