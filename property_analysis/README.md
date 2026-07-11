# Property Analysis API

![Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)
![Status](https://img.shields.io/badge/status-in%20development-yellow?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)

A REST API and analysis tool for the South African property market, focused on Cape Town and the Western Cape. Built as both a personal investment research tool and a portfolio project.

## What it does

Analyses property sales history and current listings to surface investment insights — price trends per suburb, undervalued properties, and historical sales context for specific addresses. Data is stored in PostgreSQL and seeded from scraped CSV files via a built-in CLI tool.

## Tech stack

- **Rust** — core language
- **axum** — REST API framework
- **sqlx + PostgreSQL** — persistence
- **tokio** — async runtime
- **thiserror** — structured error handling
- **jsonwebtoken** — JWT authentication
- **clap** — CLI seeding tool

## Prerequisites

- [Rust](https://rustup.rs/) — `rustc >= 1.86`
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [sqlx-cli](https://crates.io/crates/sqlx-cli) — install with:

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

## Getting started

**1. Clone the repo**
```bash
git clone https://github.com/dagancode/rust_projects.git
cd property_analysis
```

**2. Configure environment**
```bash
cp .env.example .env
```

Edit `.env` and fill in your values:

| Variable | Description |
|---|---|
| `PORT` | Port to run the API on (default: `3000`) |
| `RUST_LOG` | Log level (`debug`, `info`, `warn`, `error`) |
| `SALES_HISTORY_PATH` | Absolute path to the sales history CSV directory |
| `PROPERTY_LISTINGS_PATH` | Path to the listings CSV file |
| `JWT_SECRET` | Secret key for JWT signing — use a strong random string |
| `DATABASE_URL` | PostgreSQL connection string |

**3. Start the database**
```bash
docker compose up -d
```

**4. Run database migrations**

> sqlx-cli does not always auto-load `.env` — export `DATABASE_URL` manually first:

```bash
# PowerShell
$env:DATABASE_URL="postgresql://admin:admin@localhost:5432/property_db"

# bash / zsh
export DATABASE_URL="postgresql://admin:admin@localhost:5432/property_db"

sqlx migrate run
```

**5. Seed the database**

On first run, seed the database from your CSV files:

```bash
cargo run -- --seed
```

Additional seeding modes:

| Command | Behaviour |
|---|---|
| `cargo run -- --seed` | Seed only if tables are empty |
| `cargo run -- --seed --force` | Truncate all data and reseed |
| `cargo run -- --upsert` | Insert only rows not already present |

**6. Start the API**
```bash
cargo run
```

The API will be available at `http://localhost:3000`.

## Authentication

All `/v1/` endpoints require a JWT Bearer token. Obtain one via:

```http
POST /auth/token
Content-Type: application/json

{ "secret": "<your JWT_SECRET value>" }
```

Use the returned token as `Authorization: Bearer <token>` on subsequent requests.

## Endpoints

### Health
| Method | Path | Auth | Description |
|---|---|---|---|
| `GET` | `/health` | ❌ | Health check |

### Auth
| Method | Path | Auth | Description |
|---|---|---|---|
| `POST` | `/auth/token` | ❌ | Obtain a JWT access token |

### Sales History
| Method | Path | Auth | Description |
|---|---|---|---|
| `GET` | `/v1/sales-history/suburbs/{suburb}` | ✅ | Sales history for a suburb, with optional `?from=&to=` year filter |
| `GET` | `/v1/sales-history/properties` | ✅ | Search by `?suburb=&street=&number=` |

### Listings
| Method | Path | Auth | Description |
|---|---|---|---|
| `GET` | `/v1/listings` | ✅ | All listings, filter by `?suburb=&property_type=` |

### Analysis
| Method | Path | Auth | Description |
|---|---|---|---|
| `GET` | `/v1/analysis/suburbs/{suburb}/trends` | ✅ | Year-by-year price trend analysis with optional `?from=&to=` year filter |
| `GET` | `/v1/analysis/suburbs/{suburb}/aggregate` | ✅ | Aggregate stats (avg price, avg floor size, avg erf size, total listings) |
| `GET` | `/v1/analysis/suburbs/{suburb}/value-signals` | ✅ | Listings priced below suburb average, sorted by discount |

## Current state

Sprint 4 complete. All endpoints are backed by PostgreSQL via sqlx. Data is loaded from CSV files using the built-in CLI seeding tool with three seeding modes.

## Planned features

- Deployment via Docker + Railway/Fly.io
- TUI client built with `ratatui`
- Multi-suburb and multi-province support
- Expanded scraper coverage