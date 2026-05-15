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

The API surfaces insights such as suburb price trends, full sales history for specific properties, and value signals for underpriced listings.

**Stack:**
- `axum` - HTTP framework
- `tokio` - async runtime
- `sqlx` + PostgreSQL - persistence *(planned)*
- `thiserror` - structured domain error handling
- `reqwest` - HTTP client *(planned for scraper)*
- `rust_decimal` - monetary values (never `f64`)
- `serde` + `csv` - data ingestion

**Core questions the API answers:**
1. Is a suburb trending up over time?
2. What is the full sales history of a specific property?
3. Is a property priced below the suburb average?

**Concepts covered:**
- Axum routing, extractors, and shared application state
- `Arc<RwLock<T>>` for concurrent read access across handlers
- Structured error handling with `thiserror` and `IntoResponse`
- Type-safe money handling with `rust_decimal`
- Domain modelling and CSV ingestion pipeline
- Modular project structure (`models`, `services`, `routes`)
- Async Rust with `tokio`

**Endpoints:**
| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/suburbs/{suburb}/sales-history` | Full sales history for a suburb |

**Run the API:**
```bash
cd property_analysis
cp .env.example .env  # set SALES_HISTORY_PATH
cargo run
```

---

## Goals

- Build real systems while learning Rust fundamentals
- Progress from language basics to systems programming
- Document the learning journey through working projects

## Progress

| Project | Concepts | Status |
|---|---|---|
| txt_fighter | Traits, modules, ownership | In progress |
| property_analysis | Async, axum, concurrency, error handling | In progress |