-- Common loot items from ratting (modules, ammunition, etc.)
-- These are items that commonly drop from NPCs and can be reprocessed

INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
-- Small Weapons
(519, 'Light Neutron Blaster I', 'Small hybrid turret', 55, 8),
(3082, 'Light Ion Blaster I', 'Small hybrid turret', 55, 8),
(3074, 'Light Electron Blaster I', 'Small hybrid turret', 55, 8),
-- Medium Weapons
(520, 'Heavy Neutron Blaster I', 'Medium hybrid turret', 56, 8),
(3083, 'Heavy Ion Blaster I', 'Medium hybrid turret', 56, 8),
(3075, 'Heavy Electron Blaster I', 'Medium hybrid turret', 56, 8),
-- Large Weapons
(521, 'Neutron Blaster Cannon I', 'Large hybrid turret', 57, 8),
(3084, 'Ion Blaster Cannon I', 'Large hybrid turret', 57, 8),
(3076, 'Electron Blaster Cannon I', 'Large hybrid turret', 57, 8),
-- Armor Repairers
(3608, 'Small Armor Repairer I', 'Small armor repair module', 62, 8),
(1178, 'Medium Armor Repairer I', 'Medium armor repair module', 63, 8),
(3534, 'Large Armor Repairer I', 'Large armor repair module', 64, 8),
-- Webifiers and Scramblers
(526, 'Stasis Webifier I', 'Stasis web module', 65, 8),
(447, 'Warp Scrambler I', 'Warp scrambler module', 66, 8),
(3608, 'Warp Disruptor I', 'Warp disruptor module', 66, 8),
-- Damage Mods
(1978, 'Magnetic Field Stabilizer I', 'Hybrid damage module', 314, 8),
(2605, 'Tracking Enhancer I', 'Tracking module', 212, 8),
-- Ammunition (Hybrid Charges)
(215, 'Antimatter Charge S', 'Small antimatter charge', 83, 8),
(216, 'Antimatter Charge M', 'Medium antimatter charge', 84, 8),
(217, 'Antimatter Charge L', 'Large antimatter charge', 85, 8),
(237, 'Iridium Charge S', 'Small iridium charge', 83, 8),
(238, 'Iridium Charge M', 'Medium iridium charge', 84, 8),
(239, 'Iridium Charge L', 'Large iridium charge', 85, 8),
-- Ship Equipment
(1317, 'Afterburner I', 'Afterburner module', 46, 8),
(5973, '1MN Afterburner I', '1MN afterburner module', 46, 8),
(12076, '10MN Afterburner I', '10MN afterburner module', 46, 8)
ON CONFLICT DO NOTHING;

-- Reprocessing yields for common loot
-- These are simplified values based on typical T1 module reprocessing
INSERT INTO material_reprocessing (source_type_id, material_type_id, quantity) VALUES
-- Light Neutron Blaster I
(519, 34, 200),  -- Tritanium
(519, 35, 50),   -- Pyerite
(519, 36, 10),   -- Mexallon
-- Light Ion Blaster I
(3082, 34, 180),
(3082, 35, 45),
(3082, 36, 8),
-- Light Electron Blaster I
(3074, 34, 160),
(3074, 35, 40),
(3074, 36, 7),
-- Heavy Neutron Blaster I
(520, 34, 400),
(520, 35, 100),
(520, 36, 20),
(520, 37, 5),    -- Isogen
-- Heavy Ion Blaster I
(3083, 34, 380),
(3083, 35, 95),
(3083, 36, 18),
(3083, 37, 4),
-- Heavy Electron Blaster I
(3075, 34, 350),
(3075, 35, 90),
(3075, 36, 16),
(3075, 37, 3),
-- Neutron Blaster Cannon I
(521, 34, 800),
(521, 35, 200),
(521, 36, 40),
(521, 37, 10),
-- Ion Blaster Cannon I
(3084, 34, 750),
(3084, 35, 190),
(3084, 36, 38),
(3084, 37, 9),
-- Electron Blaster Cannon I
(3076, 34, 700),
(3076, 35, 180),
(3076, 36, 35),
(3076, 37, 8),
-- Small Armor Repairer I
(3608, 34, 150),
(3608, 35, 40),
(3608, 36, 8),
-- Medium Armor Repairer I
(1178, 34, 300),
(1178, 35, 80),
(1178, 36, 16),
(1178, 37, 4),
-- Large Armor Repairer I
(3534, 34, 600),
(3534, 35, 160),
(3534, 36, 32),
(3534, 37, 8),
-- Stasis Webifier I
(526, 34, 250),
(526, 35, 60),
(526, 36, 12),
-- Warp Scrambler I
(447, 34, 220),
(447, 35, 55),
(447, 36, 11),
-- Magnetic Field Stabilizer I
(1978, 34, 180),
(1978, 35, 45),
(1978, 36, 9),
-- Tracking Enhancer I
(2605, 34, 160),
(2605, 35, 40),
(2605, 36, 8),
-- Antimatter Charge S
(215, 34, 20),
(215, 35, 5),
-- Antimatter Charge M
(216, 34, 40),
(216, 35, 10),
-- Antimatter Charge L
(217, 34, 80),
(217, 35, 20),
(217, 36, 4),
-- Iridium Charge S
(237, 34, 18),
(237, 35, 4),
-- Iridium Charge M
(238, 34, 36),
(238, 35, 9),
-- Iridium Charge L
(239, 34, 72),
(239, 35, 18),
(239, 36, 3),
-- Afterburner I
(1317, 34, 140),
(1317, 35, 35),
-- 1MN Afterburner I
(5973, 34, 160),
(5973, 35, 40),
(5973, 36, 8),
-- 10MN Afterburner I
(12076, 34, 320),
(12076, 35, 80),
(12076, 36, 16),
(12076, 37, 4)
ON CONFLICT DO NOTHING;
