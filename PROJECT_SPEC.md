# Project: Serpentis Salvage Industrial Planner

## 🪐 Overview

This project aims to create an **industrial planning tool** for the MMO *EVE Online* that helps players analyze and optimize **salvage and loot** obtained from **Serpentis belt rats** (standard spawns).  
The goal is to identify **what items can be built** — and their **profit potential** — using the **reprocessed materials** and **salvage** collected from ratting.

The tool integrates **EVE’s ESI API**, **SDE static data**, and **market data** to provide actionable industrial insights.

---

## 🎯 Core Objectives

1. **Identify Buildable Items**  
   Match reprocessed salvage to industry blueprints and determine which items can be built (e.g., rigs, modules, ships).

2. **Optimize Resource Use**  
   Maximize ISK return by comparing the value of reprocessed minerals/salvage versus finished goods.

3. **Plan Industrial Workflows**  
   Integrate with market data, character skills, and blueprint efficiency to optimize build queues and profitability.

---

## 🧠 Conceptual Flow

Loot/Salvage → Reprocess → Materials → Blueprints → Buildable Items → Profit Ranking

---

## 🧩 Core Data Sources

| Data Type | Source | Notes |
|------------|---------|-------|
| **NPC Loot Tables** | EVE Static Data Export (SDE), Fuzzworks API | Contains drop probabilities for NPCs |
| **Salvage Items** | SDE `invTypeMaterials` | Links salvage to raw materials |
| **Blueprint Data** | SDE `industryActivityMaterials`, `industryBlueprints` | Needed to calculate material requirements |
| **Market Data** | ESI / Fuzzworks Market API | Used for Jita or custom regional price comparisons |
| **Character Skills** | ESI or manual input | Affects reprocessing yield, build efficiency, and time |

---

## 🏗️ System Architecture

### **1. Backend**

| Component | Description |
|------------|--------------|
| **Language** | Python (FastAPI) or Go |
| **Database** | SQLite / PostgreSQL for local caching of SDE and user data |
| **Data Layer** | Handles mapping of salvage → materials → blueprints |
| **API Endpoints** | JSON routes for salvage input, material output, profitability ranking |

### **2. Frontend**

| Component | Description |
|------------|--------------|
| **Framework** | React + Tailwind CSS |
| **Purpose** | User dashboard for salvage entry, buildable item lists, ISK ranking |
| **Extras** | Sort/filter UI, blueprint drill-down, “What Can I Build?” view |

---

## ⚙️ Core Engine Logic

### Step 1: Salvage → Components
- User inputs or imports their salvage items (e.g. Tripped Power Circuit x120)
- The tool looks up the corresponding material yields from the SDE

### Step 2: Components → Blueprint Matching
- Compares available materials with blueprint requirements
- Flags items that can be **fully built**, **partially built**, or **not possible**

### Step 3: Profitability Analysis
- Fetches current market prices for inputs and outputs
- Calculates:
  Build Profit = Sell Price - (Missing Material Cost + Industry Cost)
- Ranks possible builds by profitability or material efficiency

---

## 📊 Example Output

### Input Salvage
```
Tripped Power Circuit x120
Charred Micro Circuit x95
Fried Interface Circuit x40
Contaminated Nanite Compound x50
Armor Plates x10
```

### Result Table

| Item | Type | Materials Used | Missing | Est. Profit (ISK) | Build % |
|------|------|----------------|----------|--------------------|----------|
| Small Capacitor Control Circuit I | Rig | 100% match | – | 85,000 | ✅ 100% |
| Medium Nanobot Accelerator I | Rig | 75% match | 10x Armor Plates | 120,000 | ⚠️ 75% |
| Small Trimark Armor Pump I | Rig | 100% match | – | 150,000 | ✅ 100% |
| Damage Control I | Module | 60% match | 20x Tritanium, 5x Pyerite | 12,000 | ⚙️ 60% |

---

## 💡 MVP Scope (Phase 1)

**Goal:** Build a working prototype for Serpentis salvage → buildable rigs.

### Features
- Static lookup of Serpentis salvage → materials  
- Blueprint cross-matching from SDE  
- Simple ISK comparison using static market data (Fuzzworks API)  
- Input via web form or JSON  
- Output: table of buildable items and profit estimation  

---

## 🚀 Future Expansions (Phase 2+)

| Feature | Description |
|----------|--------------|
| **Faction Rat Selection** | Support other NPC factions (Guristas, Blood Raiders, Angels) |
| **Market Watch** | Track market trends for salvage vs manufactured items |
| **Skill Integration** | Modify outputs based on character’s reprocessing and industry skills |
| **Chaining Tracker Integration** | Estimate loot flow based on belt chaining efficiency |
| **Blueprint Unlock Tracker** | Suggest blueprints worth researching or buying |
| **Build Queue Simulation** | Integrate with in-game industry jobs for live planning |

---

## 🧩 Example Internal Data Mapping

| Serpentis NPC | Loot Examples | Salvage Outputs |
|----------------|----------------|-----------------|
| Chief Patroller | Light Neutron Blaster I, Armor Repairer I | Charred Micro Circuit, Tripped Power Circuit |
| Chief Safeguard | 200mm Railgun I, Armor Repairer II | Burned Logic Circuit, Contaminated Nanite Compound |
| Chief Protector | Heavy Blaster I, Web, Scram | Fried Interface Circuit, Armor Plates |
| Shadow Serpentis (Commander) | Faction Mods (Mag Stab, Sensor Booster, Web) | Intact Armor Plates, Power Circuits |

---

## 🧰 Suggested Tech Stack

| Layer | Tech | Purpose |
|--------|------|----------|
| **Backend** | Python (FastAPI) / Go | Business logic and API |
| **Database** | SQLite or PostgreSQL | Cache SDE data and user inputs |
| **Frontend** | React + Tailwind | UI for inputs and outputs |
| **External APIs** | ESI, Fuzzworks, SDE | Market prices, blueprints, loot tables |
| **Packaging** | Docker Compose | Self-host or portable deployment |

---

## 🔐 Authentication (Optional)

- Integrate **EVE SSO** for character-level skill and inventory data.
- Use tokens to retrieve:
  - Salvage materials in hangar
  - Industry job queue
  - Blueprint ownership data

---

## 🧮 Example Data Flow

```mermaid
flowchart TD
    A[User Salvage Input] --> B[Salvage Lookup (SDE)]
    B --> C[Material Yield Calculation]
    C --> D[Blueprint Matcher]
    D --> E[Market Data Query]
    E --> F[Profitability Ranking]
    F --> G[UI Display / Export CSV]
```

---

## 📦 Deliverables

| Item | Description |
|------|--------------|
| **Backend API** | REST endpoints for salvage-to-blueprint matching |
| **Frontend UI** | Simple web dashboard for salvage input/output |
| **Data Cache** | SDE + market data stored locally |
| **Documentation** | Developer setup guide and API reference |
| **Sample Dataset** | Example Serpentis belt salvage and outputs |

---

## 📅 Development Plan

| Phase | Deliverable | Estimated Time |
|--------|--------------|----------------|
| Phase 1 | MVP Salvage → Blueprint Matcher | 2 weeks |
| Phase 2 | Market Price Integration | +1 week |
| Phase 3 | Character Skill Integration | +1 week |
| Phase 4 | UI Dashboard | +2 weeks |
| Phase 5 | Expanded Faction Support | +1 week |

---

## 🪙 Long-Term Vision

The end goal is to evolve this tool into a **comprehensive industrial intelligence platform** for EVE players and corporations, combining:
- **Combat data** (from ratting)
- **Industry data** (from blueprints)
- **Market data** (regional pricing)
- **Economic predictions** (ISK/hour potential per activity)

---

## 📘 References

- [EVE Swagger Interface (ESI)](https://esi.evetech.net/)
- [Fuzzworks API](https://market.fuzzwork.co.uk/api/)
- [EVE Static Data Export (SDE)](https://developers.eveonline.com/resources)
- [EVE Ref](https://everef.net/)

---

*Authored by: Project Lead — Brian Merriam*  
*Document: `claude.md` — Version 1.0*
