################################################################################
# Stage 1 — Builder
################################################################################
FROM rustlang/rust:nightly-bookworm AS builder

# Installer cargo-binstall puis cargo-leptos (prebuilt = rapide)
RUN cargo install cargo-binstall --locked && \
    cargo binstall --no-confirm --locked cargo-leptos

# Cible WASM
RUN rustup target add wasm32-unknown-unknown

# Dépendances système pour sqlx (TLS native-roots)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier d'abord les manifestes pour caching optimal des deps
COPY Cargo.toml rust-toolchain.toml ./

# Copier le reste du projet (sources, public, migrations, style, .sqlx)
COPY . .

# sqlx offline : requiert le cache .sqlx/ committé dans le repo
ENV SQLX_OFFLINE=true

# Build release (serveur + WASM client)
RUN cargo leptos build --release -vv

################################################################################
# Stage 2 — Runtime
################################################################################
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Artefacts depuis le builder
COPY --from=builder /app/target/release/solimouv /app/solimouv
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/migrations /app/migrations

ENV LEPTOS_OUTPUT_NAME=solimouv
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV RUST_LOG=info

EXPOSE 3000

CMD ["/app/solimouv"]
