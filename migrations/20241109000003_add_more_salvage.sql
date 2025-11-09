-- Additional Serpentis salvage items
INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
(30746, 'Broken Drone Transceiver', 'Salvaged from wrecks', 966, 25),
(30744, 'Malfunctioning Shield Emitter', 'Salvaged from wrecks', 966, 25),
(30745, 'Tangled Power Conduit', 'Salvaged from wrecks', 966, 25);

-- Material reprocessing yields for new salvage items (simplified)
INSERT INTO material_reprocessing (source_type_id, material_type_id, quantity) VALUES
-- Broken Drone Transceiver
(30746, 34, 90),  -- Tritanium
(30746, 35, 45),  -- Pyerite
(30746, 36, 20),  -- Mexallon
-- Malfunctioning Shield Emitter
(30744, 34, 70),  -- Tritanium
(30744, 35, 35),  -- Pyerite
-- Tangled Power Conduit
(30745, 34, 95),  -- Tritanium
(30745, 35, 48),  -- Pyerite
(30745, 36, 22);  -- Mexallon
