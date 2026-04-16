# Solimouv' — PWA Hackathon BDCTG

> PWA de communication pour le festival sportif inclusif **Solimouv'** organisé par **Up Sport!**.
> Projet réalisé dans le cadre du hackathon BDCTG (full-Rust).

## 🛠️ Stack

- **Frontend + Backend** : [Leptos](https://leptos.dev) 0.7 (SSR + hydration WASM)
- **Serveur HTTP** : [Axum](https://github.com/tokio-rs/axum) 0.7
- **Base de données** : PostgreSQL via [sqlx](https://github.com/launchbadge/sqlx) 0.8
- **Hébergement** : [Fly.io](https://fly.io) (région `cdg` Paris) + Fly Managed Postgres
- **PWA** : manifest.json + service worker
- **CI/CD** : GitHub Actions + `flyctl deploy`

## 🌿 Git Flow

| Branche       | Rôle                                       |
| ------------- | ------------------------------------------ |
| `main`        | Production, tags sémantiques (`v0.1.0`...) |
| `develop`     | Intégration continue                       |
| `feature/*`   | Features (branchées sur `develop`)         |
| `release/*`   | Stabilisation (`develop` → `main`)         |
| `hotfix/*`    | Correctifs urgents (`main` → `main`+`develop`) |

Convention de commits : [Conventional Commits](https://www.conventionalcommits.org/).

## 🚀 Démarrage rapide

### Prérequis

```bash
# Rust nightly + cible WASM
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# Outils CLI
cargo install cargo-leptos --locked
cargo install sqlx-cli --no-default-features --features postgres
curl -L https://fly.io/install.sh | sh   # flyctl
```

### Développement local

```bash
# 1. Lancer un Postgres local (Docker)
docker run --name solimouv-pg -e POSTGRES_PASSWORD=dev -e POSTGRES_DB=solimouv -p 5432:5432 -d postgres:16

# 2. Config env
cp .env.example .env
# éditer DATABASE_URL=postgres://postgres:dev@localhost:5432/solimouv

# 3. Migrations
export DATABASE_URL="postgres://postgres:dev@localhost:5432/solimouv"
sqlx migrate run

# 4. Dev server avec hot reload
cargo leptos watch
# → http://localhost:3000
```

### Déploiement Fly.io

```bash
flyctl auth login
flyctl launch --no-deploy --name solimouv-spike --region cdg
flyctl mpg create --region cdg --name solimouv-db
flyctl secrets set DATABASE_URL="$(flyctl mpg attach solimouv-db --app solimouv-spike)"
cargo sqlx prepare -- --lib         # cache queries pour build offline
git add .sqlx && git commit -m "chore: sqlx offline cache"
flyctl deploy
flyctl open
```

## 📁 Structure

```
.
├── src/                  # Code Rust (serveur + composants Leptos)
├── public/               # Assets PWA (manifest, SW, icônes)
├── style/                # SCSS
├── migrations/           # Migrations sqlx
├── Dockerfile            # Multi-stage build Leptos
├── fly.toml              # Config Fly.io
└── Cargo.toml
```

## 📄 Licence

Projet pédagogique — hackathon BDCTG 2026.
