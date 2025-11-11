# Salvo - Serpentis Salvage Industrial Planner

An EVE Online industrial planning tool that analyzes salvage and loot from Serpentis belt rats to identify profitable manufacturing opportunities.

## Overview

Salvo helps EVE Online players answer: **"What can I build with this salvage, and is it profitable?"**

The tool processes salvage items, calculates reprocessed materials, matches them against blueprint requirements, fetches current market prices, and ranks manufacturing opportunities by profitability.

**Data Flow:** Salvage → Reprocess → Materials → Blueprint Matching → Profit Analysis

See [PROJECT_SPEC.md](PROJECT_SPEC.md) for detailed requirements and architecture.

## Features (MVP - In Development)

### Backend
- Salvage analysis API endpoint
- Material calculation from reprocessed salvage
- Blueprint matching against available materials
- Market price integration (ESI/Fuzzworks)
- Profitability ranking

### Frontend
- Modern React + TypeScript web interface
- EVE Online themed dark UI (Tailwind CSS)
- **Paste from EVE Inventory** - Copy items directly from game inventory
- Real-time salvage analysis and profitability calculations
- Interactive material and buildable items display

## Tech Stack

- **Backend:** Rust + Axum web framework
- **Frontend:** React 18 + TypeScript + Tailwind CSS v3
- **Database:** SQLite + SQLx
- **External APIs:** EVE ESI, Fuzzworks Market API

## Quick Start

### Prerequisites

- Rust 1.70+ ([install via rustup](https://rustup.rs/))
- Node.js 18+ and npm ([install via nodejs.org](https://nodejs.org/))
- SQLite 3
- Docker (optional, for containerized deployment)

### Setup

**Option 1: Local Development (Full Stack)**
```bash
# Clone the repository
git clone <repository-url>
cd salvo

# Copy environment template
cp .env.example .env

# Terminal 1: Start Backend (port 3000)
cargo run

# Terminal 2: Start Frontend (port 3001)
cd frontend
npm install
PORT=3001 npm start
```

The backend will start on `http://127.0.0.1:3000`
The frontend will start on `http://127.0.0.1:3001`

**Option 2: Backend Only (API Development)**
```bash
# Run backend server
cargo run
```

**Option 3: Docker**
```bash
# Build and run with Docker Compose
docker-compose up -d

# Or build manually
docker build -t salvo-backend .
docker run -p 3000:3000 -v $(pwd)/data:/app/data salvo-backend
```

### Verify Installation

```bash
# Backend health check
curl http://127.0.0.1:3000/health
# Expected: OK

# Backend API root
curl http://127.0.0.1:3000/
# Expected: Salvo Backend API - Serpentis Salvage Industrial Planner

# Frontend (open in browser)
# Navigate to http://localhost:3001
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

Salvo includes a Python script to import **all reprocessable items** from the official EVE SDE:

```bash
# First, run the backend once to create the database
cargo run --bin salvo-backend

# In a new terminal, run the SDE import script
python3 scripts/import_sde.py
```

The import script will:
1. Download the EVE SDE (~600MB) from CCP's official servers
2. Extract and parse the data files
3. Import 9,800+ reprocessable items and 46,000+ material yields

**What gets imported:**
- All ships, modules, ammunition, and items that can be reprocessed
- Complete reprocessing material yields from `typeMaterials.yaml`
- Item names and metadata from `types.yaml`

**Requirements:**
- Python 3.8+
- `pyyaml` and `requests` packages (usually pre-installed)
- ~1GB free disk space for the SDE data
- 2-3 minutes for import to complete

**Manual SDE download:**
If you prefer to download manually: https://developers.eveonline.com/resources/downloads

**Note:** The import script automatically skips the download if the SDE directory already exists. To force a fresh download, delete the `sde_data` directory first.

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
scripts/
  import_sde.py     - Import EVE SDE data (9,800+ items)
frontend/
  src/
    components/     - React components
    api.ts          - API client
    types.ts        - TypeScript type definitions
    App.tsx         - Main application component
  tailwind.config.js - Tailwind CSS configuration
  package.json      - Frontend dependencies
```

### Common Commands

**Backend:**
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

**Frontend:**
```bash
# Install dependencies
cd frontend
npm install

# Run development server
PORT=3001 npm start

# Build for production
npm run build

# Run tests
npm test

# Type check
npm run build
```

### Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
DATABASE_URL=sqlite:salvo.db
RUST_LOG=salvo_backend=debug,tower_http=debug
```

### Using the Frontend

**Paste from EVE Inventory Feature:**

The frontend includes a convenient feature to import salvage items directly from your EVE Online inventory:

1. In EVE Online, open your inventory and select the salvage items
2. Copy the items (Ctrl+C or Cmd+C)
3. In the Salvo web interface, click "📋 Paste from EVE Inventory"
4. Paste your clipboard contents into the text area
5. Click "Import Items"

**Supported Formats:**
```
Tripped Power Circuit x120          (x-separator)
Charred Micro Circuit  95            (multiple spaces)
Fried Interface Circuit	40           (tab-separated)
Contaminated Nanite Compound 50      (single space)
```

The parser automatically detects and handles all common EVE inventory export formats.

**CORS Configuration:**

The backend is configured with permissive CORS for development (`CorsLayer::permissive()` in `src/main.rs:37`). For production deployments, replace this with specific allowed origins:

```rust
.layer(
    CorsLayer::new()
        .allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
)
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

### Phase 1 (MVP Complete ✅)
- ✅ Core salvage analysis engine
- ✅ Blueprint matching
- ✅ Basic market integration
- ✅ Complete EVE SDE import (9,800+ items)

### Phase 2 (Current - Partially Complete)
- ✅ Frontend UI (React + Tailwind)
- ✅ Paste from EVE Inventory feature
- ✅ Full reprocessing data for all game items
- Character skill integration via ESI
- Historical market tracking
- Build queue optimization

### Phase 3
- Multi-faction support (Guristas, Blood Raiders, Angels)
- Market trend analysis and forecasting
- Corporation/alliance shared inventory
- Blueprint research tracking

## Data Sources

- **EVE SDE** - Static game data (blueprints, materials, types)
- **ESI API** - Market data and character information
- **Fuzzworks** - Aggregated market pricing

## Contributing

This is an early-stage project. Contributions welcome once MVP is complete.

## Acknowledgments

EVE Online and all related assets are property of CCP Games.
