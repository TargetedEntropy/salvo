-- EVE Online Type Definitions (items, materials, salvage, blueprints, products)
CREATE TABLE eve_types (
    type_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    group_id INTEGER,
    category_id INTEGER,
    volume REAL,
    base_price REAL,
    market_group_id INTEGER,
    portion_size INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_eve_types_name ON eve_types(name);
CREATE INDEX idx_eve_types_group ON eve_types(group_id);

-- Material reprocessing yields (what you get from reprocessing salvage/loot)
CREATE TABLE material_reprocessing (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_type_id INTEGER NOT NULL, -- The item being reprocessed
    material_type_id INTEGER NOT NULL, -- The material produced
    quantity INTEGER NOT NULL, -- Base quantity at 100% efficiency
    FOREIGN KEY (source_type_id) REFERENCES eve_types(type_id),
    FOREIGN KEY (material_type_id) REFERENCES eve_types(type_id),
    UNIQUE(source_type_id, material_type_id)
);

CREATE INDEX idx_reprocessing_source ON material_reprocessing(source_type_id);

-- Blueprint definitions
CREATE TABLE blueprints (
    blueprint_type_id INTEGER PRIMARY KEY, -- The blueprint item ID
    product_type_id INTEGER NOT NULL, -- The item this blueprint produces
    manufacturing_time INTEGER, -- Base manufacturing time in seconds
    max_production_limit INTEGER, -- Max runs per manufacturing job
    FOREIGN KEY (blueprint_type_id) REFERENCES eve_types(type_id),
    FOREIGN KEY (product_type_id) REFERENCES eve_types(type_id)
);

CREATE INDEX idx_blueprints_product ON blueprints(product_type_id);

-- Materials required for blueprint manufacturing
CREATE TABLE blueprint_materials (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    blueprint_type_id INTEGER NOT NULL,
    material_type_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL, -- Base quantity at ME 0
    FOREIGN KEY (blueprint_type_id) REFERENCES blueprints(blueprint_type_id),
    FOREIGN KEY (material_type_id) REFERENCES eve_types(type_id),
    UNIQUE(blueprint_type_id, material_type_id)
);

CREATE INDEX idx_blueprint_materials_bp ON blueprint_materials(blueprint_type_id);
CREATE INDEX idx_blueprint_materials_mat ON blueprint_materials(material_type_id);

-- Market price cache
CREATE TABLE market_prices (
    type_id INTEGER PRIMARY KEY,
    region_id INTEGER NOT NULL DEFAULT 10000002, -- Jita = 10000002
    sell_price REAL,
    buy_price REAL,
    daily_volume INTEGER,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (type_id) REFERENCES eve_types(type_id)
);

CREATE INDEX idx_market_prices_updated ON market_prices(updated_at);
