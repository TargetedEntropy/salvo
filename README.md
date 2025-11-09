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
- Docker (optional, for containerized deployment)

### Setup

**Option 1: Local Development**
```bash
# Clone the repository
git clone <repository-url>
cd salvo

# Copy environment template
cp .env.example .env

# Build and run
cargo run
```

**Option 2: Docker**
```bash
# Build and run with Docker Compose
docker-compose up -d

# Or build manually
docker build -t salvo-backend .
docker run -p 3000:3000 -v $(pwd)/data:/app/data salvo-backend
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

## Testing

### Run Tests

```bash
# Run all unit tests
cargo test

# Run integration tests
cargo test --test integration_test

# Run with output
cargo test -- --nocapture
```

### Example Scripts

```bash
# Test salvage analysis
./examples/test_salvage.sh

# Update market prices from Fuzzworks
./examples/update_prices.sh
```

## Data Import

### Import EVE SDE (Static Data Export)

The project includes sample data, but you can import real EVE data:

```bash
# Download EVE SDE (optional)
./scripts/download_sde.sh

# Import into database
cargo run --bin import_sde -- --sde-path ./sde

# Or download manually from:
# https://developers.eveonline.com/resources/downloads
```

**Note:** The SDE is large (~500MB compressed, ~2GB uncompressed). The import tool currently imports type definitions. Blueprint import is a work in progress due to the complexity of the SDE YAML structure.

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
  ],
  "reprocessing_efficiency": 0.5
}
```

**Response:**
```json
{
  "materials": [
    {"type_id": 34, "name": "Tritanium", "quantity": 1500, "unit_price": 5.5, "total_value": 8250.0}
  ],
  "total_material_value": 19755.0,
  "buildable_items": [
    {
      "product_name": "Small Capacitor Control Circuit I",
      "match_percentage": 100.0,
      "can_build": true,
      "estimated_profit": 84000.0,
      "product_price": 85000.0,
      "material_cost": 1000.0,
      "missing_materials": []
    }
  ],
  "reprocessing_efficiency_used": 0.5
}
```

### `POST /api/market/update`

Update market prices from Fuzzworks API (Jita region).

**Request:**
```json
{
  "type_ids": [34, 35, 36, 37]
}
```

**Response:**
```json
{
  "updated_count": 4,
  "prices": [
    {"type_id": 34, "sell_price": 5.5, "buy_price": 5.0}
  ]
}
```

### `POST /api/market/prices`

Get cached market prices from database.

**Request:**
```json
{
  "type_ids": [34, 35, 36]
}
```

**Response:**
```json
[
  {"type_id": 34, "sell_price": 5.5, "buy_price": 5.0, "updated_at": "2025-11-09T00:00:00Z"}
]
```

## Deployment

### Docker Deployment

The project includes Docker support for easy deployment:

```bash
# Using Docker Compose (recommended)
docker-compose up -d

# Check logs
docker-compose logs -f

# Stop
docker-compose down
```

### Environment Variables

See `.env.example` for available configuration options:
- `DATABASE_URL` - SQLite database path
- `RUST_LOG` - Logging level

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Roadmap

### Phase 1 (Current - MVP Complete ✅)
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
