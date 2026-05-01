# Property Analysis

![Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)
![Status](https://img.shields.io/badge/status-in%20development-yellow?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)

A REST API and analysis tool for the South African property market, focused on Cape Town and the Western Cape. Built as both a personal investment research tool and a portfolio project.

## What it does

Analyses property sales history and market trends to help identify valuable investment opportunities. The long-term goal is to surface insights like price trends per suburb, undervalued properties, and historical sales context for specific addresses.

## Tech stack

- **Rust** — core language
- **axum** — REST API framework
- **sqlx + PostgreSQL** — persistence
- **tokio** — async runtime
- **reqwest** — HTTP client
- **thiserror** — structured error handling

## Getting started

```bash
cp .env.example .env
# Set SALES_HISTORY_PATH in .env
cargo run
```

## Current state

Loads property sales history from CSV files and prints properties with multiple historical sales. Console output only - no API endpoints yet.

## Planned features

- REST API endpoints for querying sales history by location
- Suburb-level price trend analysis
- Undervalued property detection
- Support for multiple suburbs and provinces
- TUI for local use
- Background scraper service for real-time data updates