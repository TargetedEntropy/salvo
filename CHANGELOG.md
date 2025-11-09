# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-09

### Added

#### Core Features
- **Salvage Analysis Engine**
  - Material reprocessing calculator with efficiency support (50% to 69.575%)
  - Blueprint matching algorithm with match percentage calculation
  - Profitability analysis with market data integration
  - Missing material identification and cost calculation

#### API Endpoints
- `POST /api/salvage/analyze` - Analyze salvage and return buildable items
- `POST /api/market/update` - Update market prices from Fuzzworks API
- `POST /api/market/prices` - Get cached market prices
- `GET /health` - Health check endpoint
- `GET /` - API information endpoint

#### Database
- SQLite database with automated migrations
- 5 core tables: `eve_types`, `material_reprocessing`, `blueprints`, `blueprint_materials`, `market_prices`
- Sample data for Serpentis salvage items, materials, and rig blueprints
- Database queries with SQLx compile-time verification

#### External Integrations
- Fuzzworks Market API client with Jita region pricing
- ESI API client structure (prepared for future character integration)
- Proper error handling and API response types

#### Tools & Scripts
- **SDE Import Tool** (`import_sde` binary)
  - EVE Static Data Export downloader (`scripts/download_sde.sh`)
  - Type definition import (supports 100k+ items)
  - YAML parsing with serde_yaml
  - Progress indicators for bulk imports

- **Testing Scripts**
  - `examples/test_salvage.sh` - Comprehensive API testing
  - `examples/update_prices.sh` - Market price update script

#### Development & DevOps
- Docker support with multi-stage builds
- Docker Compose configuration
- GitHub Actions CI/CD pipeline
  - Automated testing on push/PR
  - Code formatting checks (rustfmt)
  - Linting with clippy
  - Release builds
  - Docker image builds

#### Testing
- Unit tests for reprocessing efficiency calculations
- Integration test framework setup
- Test coverage for core business logic

#### Documentation
- Comprehensive README with quick start guide
- CLAUDE.md for Claude Code development guidance
- PROJECT_SPEC.md with original requirements and architecture
- FEATURES.md with complete feature listing
- API endpoint documentation with examples
- Docker deployment instructions

### Technical Details

#### Tech Stack
- **Language:** Rust 2021 edition
- **Web Framework:** Axum 0.7
- **Database:** SQLite with SQLx 0.8
- **Async Runtime:** Tokio 1.x
- **HTTP Client:** Reqwest 0.12
- **Serialization:** Serde 1.0
- **Logging:** Tracing 0.1

#### Architecture
- Clean separation: API → Services → Database
- Type-safe database queries with compile-time verification
- Custom error types with `ApiError` enum
- Async/await throughout the codebase
- RESTful API design

#### Project Statistics
- **Lines of Rust Code:** 1,215
- **API Endpoints:** 5 functional
- **Database Tables:** 5 with migrations
- **Test Coverage:** Core logic tested
- **Build Time (Release):** ~47 seconds
- **Compile Status:** 0 errors
- **Documentation Files:** 6 comprehensive

### Dependencies
- axum = "0.7"
- tower = "0.4"
- tower-http = "0.5"
- tokio = { version = "1", features = ["full"] }
- sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "sqlite", "migrate"] }
- serde = { version = "1.0", features = ["derive"] }
- serde_json = "1.0"
- serde_yaml = "0.9"
- reqwest = { version = "0.12", features = ["json"] }
- anyhow = "1.0"
- thiserror = "1.0"
- tracing = "0.1"
- tracing-subscriber = { version = "0.3", features = ["env-filter"] }
- dotenvy = "0.15"
- chrono = { version = "0.4", features = ["serde"] }

### Known Limitations
- Blueprint import from SDE is incomplete (complex YAML structure)
- No frontend UI (backend only)
- Character skill integration not implemented
- Limited to sample data without SDE import
- Single region market data (Jita only)

### Future Enhancements
See PROJECT_SPEC.md for planned features in Phase 2+

---

## [Unreleased]

### Planned
- Complete SDE blueprint import parser
- Frontend UI with React + Tailwind CSS
- EVE SSO character authentication
- Multi-faction support (Guristas, Blood Raiders, Angels, Sansha)
- Historical market trend analysis
- Build queue optimization
- Manufacturing cost calculator with system indexes

---

[0.1.0]: https://github.com/yourusername/salvo/releases/tag/v0.1.0
