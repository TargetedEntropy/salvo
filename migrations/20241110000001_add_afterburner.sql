-- Add missing afterburner module
INSERT INTO eve_types (type_id, name, description, group_id, category_id) VALUES
(5973, '10MN Monopropellant Enduring Afterburner', 'Enduring afterburner module', 46, 8)
ON CONFLICT DO NOTHING;

-- Reprocessing yields for 10MN Monopropellant Enduring Afterburner
-- Based on similar medium-sized modules
INSERT INTO material_reprocessing (source_type_id, material_type_id, quantity) VALUES
(5973, 34, 450),   -- Tritanium
(5973, 35, 115),   -- Pyerite
(5973, 36, 23)     -- Mexallon
ON CONFLICT DO NOTHING;
