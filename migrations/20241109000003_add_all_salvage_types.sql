-- Comprehensive list of all EVE Online salvage materials
-- These are the actual salvage items that drop from wrecks

INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
-- T1 Salvage (already have some, but adding missing ones)
(30744, 'Malfunctioning Shield Emitter', 'Salvaged from wrecks', 966, 25),
(30745, 'Tangled Power Conduit', 'Salvaged from wrecks', 966, 25),
(30746, 'Broken Drone Transceiver', 'Salvaged from wrecks', 966, 25),
(30747, 'Conductive Polymer', 'Salvaged from wrecks', 966, 25),
(30748, 'Damaged Artificial Neural Network', 'Salvaged from wrecks', 966, 25),
(30749, 'Scorched Telemetry Processor', 'Salvaged from wrecks', 966, 25),
(30750, 'Thruster Console', 'Salvaged from wrecks', 966, 25),
(30751, 'Ward Console', 'Salvaged from wrecks', 966, 25),
-- T2 Salvage
(25595, 'Tripped Power Circuit', 'Salvaged from wrecks', 966, 25),
(25596, 'Charred Micro Circuit', 'Salvaged from wrecks', 966, 25),
(25597, 'Fried Interface Circuit', 'Salvaged from wrecks', 966, 25),
(25604, 'Contaminated Nanite Compound', 'Salvaged from wrecks', 966, 25),
(25605, 'Burned Logic Circuit', 'Salvaged from wrecks', 966, 25),
(25606, 'Armor Plates', 'Salvaged from wrecks', 966, 25),
(28668, 'Contaminated Lorentz Fluid', 'Salvaged from wrecks', 966, 25),
-- Ancient Salvage
(30752, 'Alloyed Tritanium Bar', 'Ancient salvage', 966, 25),
(30753, 'Carbonized Lead', 'Ancient salvage', 966, 25),
(30754, 'Conducting Polymer', 'Ancient salvage', 966, 25),
(30755, 'Construction Blocks', 'Ancient salvage', 966, 25),
(30756, 'Cramped EM Pulse Generators', 'Ancient salvage', 966, 25),
(30757, 'Defective Current Pump', 'Ancient salvage', 966, 25),
(30758, 'Single-Walled Nanotubes', 'Ancient salvage', 966, 25),
(30759, 'Smashed Trigger Unit', 'Ancient salvage', 966, 25)
ON CONFLICT DO NOTHING;

-- Material reprocessing yields for new salvage items
-- These are simplified/estimated values for testing
INSERT INTO material_reprocessing (source_type_id, material_type_id, quantity) VALUES
-- Malfunctioning Shield Emitter
(30744, 34, 70),  -- Tritanium
(30744, 35, 35),  -- Pyerite
(30744, 36, 15),  -- Mexallon
-- Tangled Power Conduit
(30745, 34, 95),  -- Tritanium
(30745, 35, 48),  -- Pyerite
(30745, 36, 22),  -- Mexallon
-- Broken Drone Transceiver
(30746, 34, 90),  -- Tritanium
(30746, 35, 45),  -- Pyerite
(30746, 36, 20),  -- Mexallon
-- Conductive Polymer
(30747, 34, 85),  -- Tritanium
(30747, 35, 42),  -- Pyerite
(30747, 36, 18),  -- Mexallon
-- Damaged Artificial Neural Network
(30748, 34, 100), -- Tritanium
(30748, 35, 60),  -- Pyerite
(30748, 36, 30),  -- Mexallon
(30748, 37, 10),  -- Isogen
-- Scorched Telemetry Processor
(30749, 34, 95),  -- Tritanium
(30749, 35, 50),  -- Pyerite
(30749, 36, 25),  -- Mexallon
-- Thruster Console
(30750, 34, 105), -- Tritanium
(30750, 35, 55),  -- Pyerite
(30750, 36, 28),  -- Mexallon
-- Ward Console
(30751, 34, 110), -- Tritanium
(30751, 35, 58),  -- Pyerite
(30751, 36, 30),  -- Mexallon
(30751, 37, 12),  -- Isogen
-- Alloyed Tritanium Bar
(30752, 34, 150), -- Tritanium
(30752, 35, 75),  -- Pyerite
-- Carbonized Lead
(30753, 34, 120), -- Tritanium
(30753, 36, 40),  -- Mexallon
-- Conducting Polymer
(30754, 34, 100), -- Tritanium
(30754, 35, 60),  -- Pyerite
(30754, 36, 35),  -- Mexallon
-- Construction Blocks
(30755, 34, 200), -- Tritanium
(30755, 35, 100), -- Pyerite
(30755, 36, 50),  -- Mexallon
-- Cramped EM Pulse Generators
(30756, 35, 80),  -- Pyerite
(30756, 36, 45),  -- Mexallon
(30756, 37, 20),  -- Isogen
-- Defective Current Pump
(30757, 34, 90),  -- Tritanium
(30757, 36, 35),  -- Mexallon
(30757, 37, 15),  -- Isogen
-- Single-Walled Nanotubes
(30758, 35, 120), -- Pyerite
(30758, 36, 60),  -- Mexallon
(30758, 37, 25),  -- Isogen
(30758, 38, 10),  -- Nocxium
-- Smashed Trigger Unit
(30759, 34, 110), -- Tritanium
(30759, 35, 70),  -- Pyerite
(30759, 36, 40),  -- Mexallon
(30759, 37, 18),  -- Isogen
-- Contaminated Lorentz Fluid
(28668, 34, 100), -- Tritanium
(28668, 35, 55),  -- Pyerite
(28668, 36, 30),  -- Mexallon
(28668, 37, 15)  -- Isogen
ON CONFLICT DO NOTHING;
