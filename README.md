# Salvo - Serpentis Salvage Industrial Planner

An EVE Online industrial planning tool that analyzes salvage and loot from Serpentis belt rats to identify profitable manufacturing opportunities.

## Overview

Salvo helps EVE Online players answer: **"What can I build with this salvage, and is it profitable?"**

The tool processes salvage items, calculates reprocessed materials, matches them against blueprint requirements, fetches current market prices, and ranks manufacturing opportunities by profitability.

**Data Flow:** Salvage → Reprocess → Materials → Blueprint Matching → Profit Analysis

See [PROJECT_SPEC.md](PROJECT_SPEC.md) for detailed requirements and architecture.

## Features (MVP - In Development)

- Salvage analysis API endpoint
- Material calculation from reprocessed salvage
- Blueprint matching against available materials
- Market price integration (ESI/Fuzzworks)
- Profitability ranking

## Tech Stack

- **Backend:** Rust + Axum web framework
- **Database:** SQLite + SQLx
- **External APIs:** EVE ESI, Fuzzworks Market API

## Quick Start

### Prerequisites

- Rust 1.70+ ([install via rustup](https://rustup.rs/))
- SQLite 3

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd salvo

# Copy environment template
cp .env.example .env

# Build and run
cargo run
```

The server will start on `http://127.0.0.1:3000`

### Verify Installation

```bash
# Health check
curl http://127.0.0.1:3000/health
# Expected: OK

# API root
curl http://127.0.0.1:3000/
# Expected: Salvo Backend API - Serpentis Salvage Industrial Planner
```

## Development

### Project Structure

```
src/
  main.rs           - Entry point, Axum server
  error.rs          - Error handling
  api/              - HTTP route handlers
  db/               - Database models and queries
  services/         - Business logic
  external/         - External API clients
migrations/         - Database migrations
```

### Common Commands

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Run with auto-reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Check code
cargo check

# Lint
cargo clippy

# Format
cargo fmt
```

### Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
DATABASE_URL=sqlite:salvo.db
RUST_LOG=salvo_backend=debug,tower_http=debug
```

## API Endpoints

### `POST /api/salvage/analyze`

Analyze salvage and return buildable items with profitability estimates.

**Request:**
```json
{
  "salvage_items": [
    {"name": "Tripped Power Circuit", "quantity": 120},
    {"name": "Charred Micro Circuit", "quantity": 95}
  ]
}
```

**Response:**
```json
{
  "materials": [
    {"name": "Tritanium", "quantity": 1500}
  ],
  "buildable_items": [
    {
      "name": "Small Capacitor Control Circuit I",
      "match_percentage": 100.0,
      "estimated_profit": 85000.0,
      "missing_materials": []
    }
  ]
}
```

## Roadmap

### Phase 1 (Current)
- Core salvage analysis engine
- Blueprint matching
- Basic market integration

### Phase 2
- Frontend UI (React + Tailwind)
- Character skill integration via ESI
- Historical market tracking

### Phase 3
- Multi-faction support (Guristas, Blood Raiders, Angels)
- Build queue simulation
- Market trend analysis

## Data Sources

- **EVE SDE** - Static game data (blueprints, materials, types)
- **ESI API** - Market data and character information
- **Fuzzworks** - Aggregated market pricing

## Contributing

This is an early-stage project. Contributions welcome once MVP is complete.

## License

TBD

## Contact

Project Lead: Brian Merriam

## Acknowledgments

EVE Online and all related assets are property of CCP Games.
