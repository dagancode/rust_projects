# rust_projects

A collection of Rust projects built while learning systems programming.

## Projects

### [txt_fighter](./txt_fighter)

A text-based RPG combat simulator built to explore Rust's trait system.

**Concepts covered:**
- Trait design and default implementations
- Dynamic dispatch with `dyn Trait`
- Ownership, borrowing, and lifetimes
- Modular project structure
- Status effect systems
- Feature flags

**Run the game:**
```bash
cd txt_fighter
cargo run

# Human-readable speed
cargo run --features slow
```

---

### [property_analysis](./property_analysis)

A REST API for analysing the South African property market, focused on Cape Town and the Western Cape. Dual purpose: portfolio project and personal investment research tool.

The API surfaces insights such as suburb price trends, full sales history for specific properties, and value signals for underpriced listings. Backed by PostgreSQL with a built-in CLI seeding tool for loading scraped property data.

**Stack:**
- `axum` — HTTP framework
- `tokio` — async runtime
- `sqlx` + PostgreSQL — persistence
- `thiserror` — structured domain error handling
- `jsonwebtoken` — JWT authentication
- `clap` — CLI seeding tool
- `rust_decimal` — monetary values (never `f64`)
- `serde` + `csv` — data ingestion pipeline

**Core questions the API answers:**
1. Is a suburb trending up over time?
2. What is the full sales history of a specific property?
3. Is a property priced below the suburb average?

**Concepts covered:**
- Axum routing, extractors, middleware, and shared application state
- JWT authentication via `from_fn_with_state` middleware
- `sqlx` with compile-time checked queries against PostgreSQL
- Structured error handling with `thiserror` and `IntoResponse`
- Type-safe money handling with `rust_decimal`
- CLI tooling with `clap` — multiple seeding modes with transaction management
- Docker Compose for local development
- Domain modelling and CSV ingestion pipeline
- Modular project structure (`models`, `services`, `routes`, `db`)
- Async Rust with `tokio`

**Endpoints:**
| Method | Path | Auth | Description |
|---|---|---|---|
| `GET` | `/health` | ❌ | Health check |
| `POST` | `/auth/token` | ❌ | Obtain a JWT access token |
| `GET` | `/v1/sales-history/suburbs/{suburb}` | ✅ | Sales history for a suburb with optional year filter |
| `GET` | `/v1/sales-history/properties` | ✅ | Search by suburb, street, and number |
| `GET` | `/v1/listings` | ✅ | Listings filtered by suburb and property type |
| `GET` | `/v1/analysis/suburbs/{suburb}/trends` | ✅ | Year-by-year price trend analysis |
| `GET` | `/v1/analysis/suburbs/{suburb}/aggregate` | ✅ | Aggregate suburb stats |
| `GET` | `/v1/analysis/suburbs/{suburb}/value-signals` | ✅ | Listings priced below suburb average |

**Run the API:**
```bash
cd property_analysis
cp .env.example .env       # configure environment
docker compose up -d       # start PostgreSQL
cargo run -- --seed        # seed database from CSV
cargo run                  # start the API
```

See [property_analysis/README.md](./property_analysis/README.md) for full setup instructions.

---

## Goals

- Build real systems while learning Rust fundamentals
- Progress from language basics to systems programming
- Document the learning journey through working projects

## Progress

| Project | Concepts | Status |
|---|---|---|
| txt_fighter | Traits, modules, ownership | In progress |
| property_analysis | Async, axum, PostgreSQL, JWT auth, CLI tooling | In progress |