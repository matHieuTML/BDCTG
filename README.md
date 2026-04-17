# Solimouv' — PWA du festival sportif inclusif

> **Application web progressive** pour le festival **Solimouv'**, organisé par l'association **Up Sport!** (Paris).
> Projet réalisé dans le cadre du hackathon BDCTG 2026 — stack **full-Rust**.

**Production** : [https://solimouv-spike.fly.dev/](https://solimouv-spike.fly.dev/)

---

## Table des matières

1. [Présentation du projet](#présentation-du-projet)
2. [Architecture technique](#architecture-technique)
3. [Stack technologique](#stack-technologique)
4. [Structure du dépôt](#structure-du-dépôt)
5. [Base de données](#base-de-données)
6. [Installation et développement local](#installation-et-développement-local)
7. [Déploiement](#déploiement)
8. [Accès et identifiants](#accès-et-identifiants)
9. [Fonctionnalités](#fonctionnalités)
10. [Git Flow](#git-flow)
11. [API — Server Functions](#api--server-functions)

---

## Présentation du projet

Solimouv' est le festival du sport pour tous, organisé par Up Sport! et un collectif de 13 associations parisiennes. Cette PWA sert de **plateforme de communication et d'inscription** pour l'événement :

- Présentation du festival et de l'association Up Sport!
- Programme des 8 ateliers sportifs
- Inscription à l'événement avec génération de QR code
- Espace personnel utilisateur et dashboard administrateur
- Pages partenaires et contact
- Application installable sur mobile (PWA)

---

## Architecture technique

```
┌─────────────────────────────────────────────────────┐
│                    Client (navigateur)               │
│         WASM hydration + Service Worker (PWA)        │
└──────────────────────┬──────────────────────────────┘
                       │ HTTPS
┌──────────────────────▼──────────────────────────────┐
│                  Fly.io (région cdg — Paris)          │
│  ┌───────────────────────────────────────────────┐   │
│  │           Axum 0.7 (serveur HTTP)             │   │
│  │  ┌─────────────┐  ┌──────────────────────┐    │   │
│  │  │ Leptos 0.7  │  │  Server Functions     │    │   │
│  │  │ SSR + HTML  │  │  (auth, inscriptions) │    │   │
│  │  └─────────────┘  └──────────┬───────────┘    │   │
│  └──────────────────────────────┼────────────────┘   │
└─────────────────────────────────┼────────────────────┘
                                  │ TCP/SSL
┌─────────────────────────────────▼────────────────────┐
│            Supabase PostgreSQL (EU West)              │
│      users · sessions · inscriptions · health_check  │
└──────────────────────────────────────────────────────┘
```

**Principe** : Leptos effectue le rendu HTML côté serveur (SSR), puis le navigateur télécharge le bundle WASM pour l'hydration interactive. Les server functions (marquées `#[server]`) s'exécutent uniquement côté serveur et communiquent avec PostgreSQL via sqlx.

---

## Stack technologique

| Couche | Technologie | Version | Rôle |
|--------|-------------|---------|------|
| Frontend + SSR | Leptos | 0.7.8 | Composants réactifs, SSR, hydration WASM |
| Routeur | leptos_router | 0.7 | Routage client/serveur |
| SEO | leptos_meta | 0.7 | Balises meta, Open Graph, Twitter Cards |
| Serveur HTTP | Axum | 0.7 | Routage HTTP, middleware, static files |
| Runtime async | Tokio | 1.x | Runtime asynchrone |
| Base de données | PostgreSQL | 15 | Stockage persistant |
| ORM / Queries | sqlx | 0.8 | Requêtes SQL typées à l'exécution |
| Authentification | bcrypt | 0.16 | Hash des mots de passe (coût 12) |
| QR Code | qrcode | 0.14 | Génération SVG côté serveur |
| Styles | SCSS | — | Design system (Clash Display + DM Sans) |
| Hébergement | Fly.io | — | Containers Docker, région Paris (cdg) |
| BDD managée | Supabase | — | PostgreSQL gratuit, EU West |
| PWA | manifest.json + SW | — | Installation mobile, cache offline |
| Langage | Rust | nightly-2026-04-16 | Full-stack (serveur + client WASM) |

---

## Structure du dépôt

```
BDCTG/
├── src/
│   ├── main.rs                    # Point d'entrée serveur Axum
│   ├── lib.rs                     # Re-exports modules
│   ├── app.rs                     # Composant racine + routeur Leptos
│   ├── auth.rs                    # Server functions auth (login, register, logout)
│   ├── inscriptions.rs            # Server functions inscriptions + QR code
│   ├── data.rs                    # Données statiques des 8 ateliers
│   ├── models.rs                  # Structs partagées (User)
│   ├── fileserv.rs                # Handler fichiers statiques
│   ├── components/
│   │   ├── header.rs              # Header + navigation + hamburger mobile
│   │   ├── footer.rs              # Footer
│   │   ├── layout.rs              # Layout commun (header + main + footer)
│   │   └── seo.rs                 # Composant SEO (meta, OG, canonical)
│   ├── pages/
│   │   ├── home.rs                # Page d'accueil + Schema.org
│   │   ├── about.rs               # À propos (Up Sport! + Solimouv')
│   │   ├── programme.rs           # Programme des 8 ateliers
│   │   ├── associations.rs        # Logos partenaires
│   │   ├── contact.rs             # Formulaire de contact
│   │   ├── login.rs               # Page connexion
│   │   ├── register.rs            # Page inscription
│   │   ├── dashboard.rs           # Espace personnel (dispatch user/admin)
│   │   ├── admin_dashboard.rs     # Dashboard admin (stats, graphiques)
│   │   └── not_found.rs           # Page 404
│   └── server/
│       ├── db.rs                  # Pool PostgreSQL + init
│       ├── auth.rs                # Logique auth côté serveur
│       └── seo.rs                 # Sitemap XML
├── migrations/
│   ├── 20260416000001_init.sql                # Table health_check
│   ├── 20260417000001_users_sessions.sql      # Tables users + sessions
│   ├── 20260417000002_inscriptions.sql        # Table inscriptions (v1)
│   └── 20260417000003_simplify_inscriptions.sql # Simplification (1 user = 1 inscription)
├── public/
│   ├── manifest.json              # Manifest PWA
│   ├── sw.js                      # Service Worker (cache offline)
│   ├── favicon.ico                # Favicon
│   ├── LogoHeader.png             # Logo header
│   ├── icons/                     # Icônes PWA (192, 512, apple-touch)
│   ├── about/                     # Photo de groupe
│   └── asso-logos/                # Logos partenaires
├── style/
│   └── main.scss                  # Design system complet (~2700 lignes)
├── Cargo.toml                     # Dépendances Rust
├── Cargo.lock                     # Lock file
├── Dockerfile                     # Build multi-stage (builder + runtime)
├── fly.toml                       # Configuration Fly.io
├── rust-toolchain.toml            # Pin nightly + target wasm32
├── .env.example                   # Template variables d'environnement
├── .gitignore                     # Fichiers exclus du versioning
└── .dockerignore                  # Fichiers exclus du build Docker
```

---

## Base de données

PostgreSQL 15 hébergé sur **Supabase** (région EU West). 4 tables :

### `users`

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| id | UUID | PK, default gen_random_uuid() | Identifiant unique |
| email | TEXT | UNIQUE, NOT NULL | Adresse e-mail |
| prenom | TEXT | NOT NULL | Prénom |
| password_hash | TEXT | NOT NULL | Hash bcrypt (coût 12) |
| role | TEXT | NOT NULL, CHECK (admin\|user), default 'user' | Rôle |
| created_at | TIMESTAMPTZ | NOT NULL, default NOW() | Date création |

### `sessions`

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| id | UUID | PK | Identifiant de session |
| user_id | UUID | FK → users(id) CASCADE | Référence utilisateur |
| created_at | TIMESTAMPTZ | NOT NULL | Date création |
| expires_at | TIMESTAMPTZ | NOT NULL, default NOW() + 7j | Expiration |

### `inscriptions`

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| id | UUID | PK | Identifiant |
| user_id | UUID | UNIQUE, FK → users(id) CASCADE | 1 inscription / utilisateur |
| created_at | TIMESTAMPTZ | NOT NULL | Date inscription |

### `health_check`

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| id | SERIAL | PK | Auto-incrémenté |
| checked_at | TIMESTAMPTZ | NOT NULL, index DESC | Horodatage |

### Relations

```
users ──┬── 1:N ──→ sessions       (ON DELETE CASCADE)
        └── 1:1 ──→ inscriptions   (ON DELETE CASCADE)
```

---

## Installation et développement local

### Prérequis

- **Rust nightly** (>= 2026-04-16) + target `wasm32-unknown-unknown`
- **PostgreSQL** 15+ (local ou Docker)
- **cargo-leptos** (build tool Leptos)

```bash
# Installer Rust nightly + WASM
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# Installer cargo-leptos
cargo install cargo-leptos --locked

# (Optionnel) sqlx-cli pour les migrations manuelles
cargo install sqlx-cli --no-default-features --features postgres
```

### Lancer en local

```bash
# 1. PostgreSQL via Docker
docker run --name solimouv-pg \
  -e POSTGRES_PASSWORD=dev \
  -e POSTGRES_DB=solimouv \
  -p 5432:5432 -d postgres:16

# 2. Configurer l'environnement
cp .env.example .env
# Éditer .env : DATABASE_URL=postgres://postgres:dev@localhost:5432/solimouv

# 3. Appliquer les migrations
export DATABASE_URL="postgres://postgres:dev@localhost:5432/solimouv"
sqlx migrate run

# 4. Lancer le serveur de développement (hot reload)
cargo leptos watch
```

L'application est disponible sur **http://localhost:3000**.

---

## Déploiement

### Infrastructure de production

| Service | Fournisseur | Région | Détails |
|---------|-------------|--------|---------|
| Application | Fly.io | cdg (Paris) | 2 VMs shared-cpu-1x, 512 MB RAM, auto-stop |
| Base de données | Supabase | EU West | PostgreSQL 15, plan gratuit (500 MB) |
| HTTPS | Fly.io | — | Certificat TLS automatique |
| DNS | Fly.io | — | solimouv-spike.fly.dev |

### Procédure de déploiement

```bash
# 1. Se connecter à Fly.io
flyctl auth login

# 2. Configurer le secret DATABASE_URL
flyctl secrets set DATABASE_URL="postgresql://..." --app solimouv-spike

# 3. Déployer (build Docker distant)
flyctl deploy

# 4. Vérifier
flyctl status
flyctl open    # ouvre https://solimouv-spike.fly.dev/
flyctl logs    # logs en temps réel
```

Le `Dockerfile` utilise un build multi-stage :
1. **Builder** : image `rustlang/rust:nightly-bookworm`, compile le serveur Rust + le bundle WASM via `cargo leptos build --release`
2. **Runtime** : image `debian:bookworm-slim` minimale (~80 MB), exécute le binaire compilé

---

## Accès et identifiants

### Comptes applicatifs

| Rôle | Email | Mot de passe | Accès |
|------|-------|-------------|-------|
| Administrateur | admin@solimouv.fr | Admin123! | Dashboard admin (/mon-espace) |
| Utilisateur test | *(créer via /inscription)* | — | Dashboard user + QR code |

### Services externes

| Service | URL | Compte |
|---------|-----|--------|
| GitHub | github.com/matHieuTML/BDCTG | matHieuTML |
| Fly.io | fly.io/apps/solimouv-spike | gaucher.mathieu@icloud.com |
| Supabase | supabase.com (projet axhsyerwmhftdrkjjonm) | gaucher.mathieu@icloud.com |

### Variables d'environnement

Copier `.env.example` et renseigner :

```env
# Base de données PostgreSQL (Supabase Session Pooler)
DATABASE_URL=postgresql://postgres.[ref]:[PASSWORD]@aws-0-eu-west-1.pooler.supabase.com:5432/postgres

# Leptos
LEPTOS_OUTPUT_NAME=solimouv
LEPTOS_SITE_ROOT=target/site
LEPTOS_SITE_PKG_DIR=pkg
LEPTOS_SITE_ADDR=0.0.0.0:3000
```

> **Sécurité** : le fichier `.env` est exclu du versioning (`.gitignore`). Les secrets de production sont injectés via `flyctl secrets set` et ne sont jamais stockés dans le code.

---

## Fonctionnalités

| Fonctionnalité | Statut | Description |
|----------------|--------|-------------|
| Pages vitrine (accueil, à propos, programme, associations, contact) | Livré | 5 pages SSR avec SEO complet |
| Authentification (inscription, connexion, déconnexion) | Livré | bcrypt + sessions cookie (7 jours) |
| Inscription événement | Livré | 1 inscription globale par utilisateur |
| QR code | Livré | Génération SVG côté serveur |
| Dashboard utilisateur | Livré | Statut inscription + QR code |
| Dashboard administrateur | Livré | KPIs, graphiques par atelier, alertes |
| PWA installable | Livré | Manifest + Service Worker + icônes |
| Responsive mobile | Livré | Hamburger menu, breakpoints 768/1024px |
| Accessibilité WCAG AA | Livré | Contraste, ARIA, sémantique HTML, skip-link |
| SEO | Livré | Meta, Open Graph, Twitter Cards, Schema.org, canonical |
| Déploiement Fly.io | Livré | Docker multi-stage, HTTPS, auto-stop |

---

## Git Flow

Le projet suit le modèle **GitFlow** (Vincent Driessen) :

```
main ────────────●──────────── (tag v0.1.0)
                 │
release/v0.1.0 ──●
                 │
develop ─────────●
                 │
feature/leptos-spike ──●──●──●──●──●
```

| Branche | Rôle |
|---------|------|
| `main` | Production, tags sémantiques (v0.1.0) |
| `develop` | Intégration continue |
| `feature/*` | Développement de fonctionnalités |
| `release/*` | Stabilisation avant mise en production |
| `hotfix/*` | Correctifs urgents |

**Convention de commits** : [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `chore:`, `docs:`).

---

## API — Server Functions

Les interactions client-serveur passent par des **server functions** Leptos (RPC automatique, pas de REST manuel) :

| Fonction | Endpoint | Description |
|----------|----------|-------------|
| `register` | POST /api/register | Création de compte (email, prénom, mot de passe) |
| `login` | POST /api/login | Connexion (email + mot de passe → cookie session) |
| `logout` | POST /api/logout | Déconnexion (suppression session) |
| `get_current_user` | POST /api/get_current_user | Récupérer l'utilisateur connecté |
| `join_event` | POST /api/join_event | S'inscrire à l'événement |
| `is_registered` | POST /api/is_registered | Vérifier si inscrit |
| `generate_qr_svg` | POST /api/generate_qr_svg | Générer le QR code SVG |

---

## Licence

Projet pédagogique — Hackathon BDCTG 2026, Up Sport!
