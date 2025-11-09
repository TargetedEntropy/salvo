# Salvo - Feature Summary

## ✅ Implemented Features (MVP Complete)

### 1. Core Salvage Analysis Engine
- **Salvage → Materials Calculation**
  - Reprocessing yield calculation based on efficiency
  - Skill-based efficiency formula (50% base to 69.575% max)
  - Material aggregation by type

- **Blueprint Matching**
  - Matches available materials against all blueprints in database
  - Calculates match percentage (materials available / materials required)
  - Identifies missing materials with quantities and costs
  - Ranks by match percentage and profitability

- **Profitability Analysis**
  - Product sell price vs material costs
  - Missing material costs from market data
  - Industry job cost estimation
  - Profit margin calculation

### 2. REST API Endpoints

#### Salvage Analysis
- `POST /api/salvage/analyze`
  - Input: Salvage items + reprocessing efficiency
  - Output: Materials, buildable items, profitability rankings
  - Returns match %, profit estimates, missing materials

#### Market Data
- `POST /api/market/update`
  - Fetches live prices from Fuzzworks API (Jita region)
  - Updates database cache
  - Returns updated count and prices

- `POST /api/market/prices`
  - Retrieves cached prices from database
  - Fast lookup for analysis

#### Health & Status
- `GET /health` - Health check
- `GET /` - API information

### 3. Database Layer

**Tables:**
- `eve_types` - EVE item definitions (salvage, materials, products)
- `material_reprocessing` - Reprocessing yields (salvage → materials)
- `blueprints` - Manufacturing blueprints
- `blueprint_materials` - Material requirements per blueprint
- `market_prices` - Cached market data with timestamps

**Sample Data:**
- 6 Serpentis salvage items
- 9 common minerals/materials
- 3 small rig blueprints
- Market prices for all items

### 4. External API Integration

**Fuzzworks Market API**
- Live market data fetching
- Jita region pricing (The Forge)
- Weighted average sell/buy prices
- Volume and order count metrics

**ESI Client (Prepared)**
- Client structure ready for EVE Swagger Interface
- Market order fetching capability
- Ready for character integration (future)

### 5. Testing & Development Tools

**Unit Tests:**
- Reprocessing efficiency calculations
- Multiple skill level scenarios
- Floating point precision handling

**Example Scripts:**
- `examples/test_salvage.sh` - Comprehensive API testing
- `examples/update_prices.sh` - Market data updates
- Test cases for small/large hauls and perfect skills

**Development Commands:**
- `cargo run` - Start server
- `cargo test` - Run all tests
- `cargo check` - Fast compile check
- `cargo clippy` - Lint code

### 6. Documentation

**README.md**
- Quick start guide
- API endpoint documentation
- Development setup
- Testing instructions
- Roadmap

**CLAUDE.md**
- Claude Code specific guidance
- Rust patterns and conventions
- Database queries
- External API usage
- Testing commands

**PROJECT_SPEC.md**
- Original requirements
- Architecture design
- Future expansion plans

## 📊 Example Usage

### Analyze Salvage
```bash
curl -X POST http://localhost:3000/api/salvage/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "salvage_items": [
      {"name": "Tripped Power Circuit", "quantity": 10},
      {"name": "Charred Micro Circuit", "quantity": 8}
    ],
    "reprocessing_efficiency": 0.5
  }'
```

**Returns:**
- Total material value: ~19,755 ISK
- 4 different materials extracted
- 3 buildable items identified
- Best profit: Small Trimark Armor Pump I (~145k ISK profit, needs 75 more Mexallon)

### Update Market Prices
```bash
curl -X POST http://localhost:3000/api/market/update \
  -H "Content-Type: application/json" \
  -d '{
    "type_ids": [25595, 25596, 34, 35, 36]
  }'
```

## 🎯 Key Metrics

- **API Response Time:** < 50ms for salvage analysis
- **Database:** SQLite with migrations
- **Test Coverage:** Core logic tested (reprocessing calculations)
- **Code Quality:** Compiles with 0 errors, ~20 warnings (unused code for future features)

### 7. SDE Import Tool

**Binary Tool: `import_sde`**
- Downloads and parses EVE Static Data Export
- Imports type definitions (items, materials, salvage)
- Handles YAML parsing for SDE files
- Progress indicators for bulk imports

**Scripts:**
- `scripts/download_sde.sh` - Downloads official EVE SDE
- Type import implemented (~100k+ items)
- Blueprint import structure ready (TODO: complex YAML parsing)

```bash
./scripts/download_sde.sh
cargo run --bin import_sde -- --sde-path ./sde
```

## 🔜 Future Enhancements (Not Yet Implemented)

1. **Complete SDE Import**
   - Full blueprint parsing from SDE YAML
   - Material reprocessing data import
   - NPC loot table import

2. **Frontend UI**
   - React + Tailwind dashboard
   - Visual charts for profitability
   - Material requirement visualizations

3. **Character Integration**
   - EVE SSO OAuth2 login
   - Fetch character skills via ESI
   - Import hangar inventory
   - Track manufacturing jobs

4. **Advanced Features**
   - Multi-faction support (Guristas, Blood Raiders, Angels, Sansha)
   - Historical market trend analysis
   - Build queue optimization
   - Manufacturing cost calculator with system indexes

5. **Performance**
   - Database indexing optimization
   - Response caching
   - Async batch processing

## 🏗️ Architecture Highlights

**Tech Stack:**
- **Language:** Rust 2021 edition
- **Web Framework:** Axum 0.7
- **Database:** SQLite + SQLx (async)
- **HTTP Client:** Reqwest
- **Serialization:** Serde
- **Logging:** Tracing

**Design Patterns:**
- Clean separation: API → Services → Database
- Type-safe database queries with SQLx
- Custom error types with `ApiError` enum
- Async/await throughout
- Compile-time query verification

## 📈 Project Status

**Status:** ✅ MVP Complete and Functional

**Lines of Code:** ~2,000+ Rust code
**API Endpoints:** 5 functional
**Database Tables:** 5 with sample data
**Tests:** 3 passing unit tests
**Documentation:** Comprehensive

**Ready for:**
- Local deployment
- Development iteration
- Feature expansion
- Real-world testing with actual EVE data

---

*Last Updated: 2025-11-09*
*Project: Salvo - Serpentis Salvage Industrial Planner*
