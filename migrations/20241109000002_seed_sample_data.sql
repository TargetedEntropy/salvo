-- Sample EVE Types for MVP testing
-- Note: These are approximations for development. Real type_ids should come from SDE import.

-- Salvage items (Serpentis)
INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
(25595, 'Tripped Power Circuit', 'Salvaged from wrecks', 966, 25), -- Salvage
(25596, 'Charred Micro Circuit', 'Salvaged from wrecks', 966, 25),
(25597, 'Fried Interface Circuit', 'Salvaged from wrecks', 966, 25),
(25604, 'Contaminated Nanite Compound', 'Salvaged from wrecks', 966, 25),
(25606, 'Armor Plates', 'Salvaged from wrecks', 966, 25),
(25605, 'Burned Logic Circuit', 'Salvaged from wrecks', 966, 25);

-- Common minerals/components that salvage reprocesses into
INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
(34, 'Tritanium', 'Base mineral', 18, 4),
(35, 'Pyerite', 'Base mineral', 18, 4),
(36, 'Mexallon', 'Base mineral', 18, 4),
(37, 'Isogen', 'Base mineral', 18, 4),
(38, 'Nocxium', 'Base mineral', 18, 4),
(39, 'Zydrine', 'Base mineral', 18, 4),
(40, 'Megacyte', 'Base mineral', 18, 4),
(44, 'Enriched Uranium', 'Processed material', 423, 4),
(11399, 'Morphite', 'Rare mineral', 18, 4);

-- Rig components (what salvage commonly reprocesses into)
INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
(25613, 'Salvaged Materials', 'Processed salvage material', 964, 4),
(25614, 'Lorentz Fluid', 'Processed salvage material', 964, 4);

-- Sample rig blueprints
INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
(26906, 'Small Capacitor Control Circuit I Blueprint', 'Rig blueprint', 1332, 9),
(26912, 'Small Trimark Armor Pump I Blueprint', 'Rig blueprint', 1332, 9),
(26910, 'Small Nanobot Accelerator I Blueprint', 'Rig blueprint', 1332, 9);

-- Sample rig products
INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
(26898, 'Small Capacitor Control Circuit I', 'Ship rig', 1137, 6),
(26904, 'Small Trimark Armor Pump I', 'Ship rig', 1137, 6),
(26902, 'Small Nanobot Accelerator I', 'Ship rig', 1137, 6);

-- Material reprocessing yields (simplified for MVP)
-- Format: source_item -> material + quantity (at 100% efficiency)
INSERT INTO material_reprocessing (source_type_id, material_type_id, quantity) VALUES
-- Tripped Power Circuit
(25595, 34, 100), -- Tritanium
(25595, 35, 50),  -- Pyerite
(25595, 36, 25),  -- Mexallon
-- Charred Micro Circuit
(25596, 34, 80),
(25596, 35, 60),
(25596, 37, 15),  -- Isogen
-- Fried Interface Circuit
(25597, 35, 100),
(25597, 36, 50),
(25597, 37, 20),
-- Contaminated Nanite Compound
(25604, 36, 75),
(25604, 37, 40),
(25604, 38, 10),  -- Nocxium
-- Armor Plates
(25606, 34, 150),
(25606, 36, 100),
-- Burned Logic Circuit
(25605, 35, 90),
(25605, 37, 45),
(25605, 38, 15);

-- Blueprint definitions
INSERT INTO blueprints (blueprint_type_id, product_type_id, manufacturing_time, max_production_limit) VALUES
(26906, 26898, 600, 10),   -- Small Capacitor Control Circuit I
(26912, 26904, 600, 10),   -- Small Trimark Armor Pump I
(26910, 26902, 600, 10);   -- Small Nanobot Accelerator I

-- Blueprint material requirements (simplified)
INSERT INTO blueprint_materials (blueprint_type_id, material_type_id, quantity) VALUES
-- Small Capacitor Control Circuit I
(26906, 34, 500),  -- Tritanium
(26906, 35, 300),  -- Pyerite
(26906, 36, 100),  -- Mexallon
-- Small Trimark Armor Pump I
(26912, 34, 800),
(26912, 35, 400),
(26912, 36, 200),
(26912, 37, 50),   -- Isogen
-- Small Nanobot Accelerator I
(26910, 35, 500),
(26910, 36, 300),
(26910, 37, 100),
(26910, 38, 25);   -- Nocxium

-- Sample market prices (Jita estimates in ISK)
INSERT INTO market_prices (type_id, region_id, sell_price, buy_price, daily_volume) VALUES
-- Salvage
(25595, 10000002, 15000, 12000, 5000),  -- Tripped Power Circuit
(25596, 10000002, 12000, 9500, 4500),   -- Charred Micro Circuit
(25597, 10000002, 18000, 15000, 3000),  -- Fried Interface Circuit
(25604, 10000002, 25000, 20000, 2000),  -- Contaminated Nanite Compound
(25606, 10000002, 30000, 25000, 1500),  -- Armor Plates
(25605, 10000002, 14000, 11000, 3500),  -- Burned Logic Circuit
-- Minerals
(34, 10000002, 5.5, 5.0, 1000000000),   -- Tritanium
(35, 10000002, 8.0, 7.5, 500000000),    -- Pyerite
(36, 10000002, 45.0, 42.0, 200000000),  -- Mexallon
(37, 10000002, 95.0, 90.0, 100000000),  -- Isogen
(38, 10000002, 650.0, 620.0, 50000000), -- Nocxium
(39, 10000002, 1200.0, 1150.0, 20000000), -- Zydrine
(40, 10000002, 2500.0, 2400.0, 10000000), -- Megacyte
-- Rigs (manufactured items)
(26898, 10000002, 85000, 75000, 1000),   -- Small Capacitor Control Circuit I
(26904, 10000002, 150000, 130000, 800),  -- Small Trimark Armor Pump I
(26902, 10000002, 120000, 100000, 900);  -- Small Nanobot Accelerator I
