-- Missing salvage items and common meta module variants
-- These are items commonly found in ratting loot

INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
-- Missing Salvage Items (T2/T1 Salvage)
(25598, 'Interface Circuit', 'Salvaged from wrecks', 966, 25),
(30743, 'Telemetry Processor', 'Salvaged from wrecks', 966, 25),

-- Meta Module Variants (Compact/Scoped/Enduring versions)
-- Guidance Disruptors
(2553, 'Highstroke Scoped Guidance Disruptor', 'Scoped tracking disruptor', 213, 8),
(4027, 'C-IR Compact Guidance Disruptor', 'Compact tracking disruptor', 213, 8),

-- Armor Repairers (Meta variants)
(41390, 'Small I-b Polarized Structural Regenerator', 'Polarized armor repairer', 62, 8),
(5975, 'Large Solace Scoped Remote Armor Repairer', 'Scoped remote armor repairer', 325, 8),

-- Webifiers
(527, 'X5 Enduring Stasis Webifier', 'Enduring stasis web', 65, 8),

-- Energy Neutralizers
(2897, 'Small Infectious Scoped Energy Neutralizer', 'Scoped energy neutralizer', 71, 8),

-- Laser Weapons
(3544, 'Small Focused Modal Laser I', 'Small laser turret', 53, 8),

-- ECM Modules
(1953, 'Hypnos Scoped Magnetometric ECM', 'Scoped ECM module', 202, 8),

-- Warp Scramblers
(448, 'Initiated Compact Warp Scrambler', 'Compact warp scrambler', 66, 8)
ON CONFLICT DO NOTHING;

-- Reprocessing yields for missing salvage items
INSERT INTO material_reprocessing (source_type_id, material_type_id, quantity) VALUES
-- Interface Circuit (similar to other circuits)
(25598, 34, 100),  -- Tritanium
(25598, 35, 52),   -- Pyerite
(25598, 36, 28),   -- Mexallon
(25598, 37, 12),   -- Isogen

-- Telemetry Processor (similar to Scorched Telemetry Processor)
(30743, 34, 95),   -- Tritanium
(30743, 35, 50),   -- Pyerite
(30743, 36, 25),   -- Mexallon

-- Highstroke Scoped Guidance Disruptor
(2553, 34, 240),   -- Tritanium
(2553, 35, 60),    -- Pyerite
(2553, 36, 12),    -- Mexallon

-- C-IR Compact Guidance Disruptor
(4027, 34, 220),   -- Tritanium
(4027, 35, 55),    -- Pyerite
(4027, 36, 11),    -- Mexallon

-- Small I-b Polarized Structural Regenerator
(41390, 34, 160),  -- Tritanium
(41390, 35, 42),   -- Pyerite
(41390, 36, 9),    -- Mexallon

-- Large Solace Scoped Remote Armor Repairer
(5975, 34, 650),   -- Tritanium
(5975, 35, 170),   -- Pyerite
(5975, 36, 35),    -- Mexallon
(5975, 37, 9),     -- Isogen

-- X5 Enduring Stasis Webifier
(527, 34, 260),    -- Tritanium
(527, 35, 65),     -- Pyerite
(527, 36, 13),     -- Mexallon

-- Small Infectious Scoped Energy Neutralizer
(2897, 34, 190),   -- Tritanium
(2897, 35, 48),    -- Pyerite
(2897, 36, 10),    -- Mexallon

-- Small Focused Modal Laser I
(3544, 34, 180),   -- Tritanium
(3544, 35, 45),    -- Pyerite
(3544, 36, 9),     -- Mexallon

-- Hypnos Scoped Magnetometric ECM
(1953, 34, 230),   -- Tritanium
(1953, 35, 58),    -- Pyerite
(1953, 36, 12),    -- Mexallon

-- Initiated Compact Warp Scrambler
(448, 34, 230),    -- Tritanium
(448, 35, 58),     -- Pyerite
(448, 36, 12)     -- Mexallon

ON CONFLICT DO NOTHING;
