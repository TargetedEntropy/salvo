#!/usr/bin/env python3
"""
Import EVE SDE data into the Salvo database.
Downloads and parses the EVE Static Data Export to populate item types and reprocessing data.
"""

import requests
import yaml
import sqlite3
import os
import zipfile
from pathlib import Path

SDE_URL = "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip"
SDE_DIR = Path("./sde_data")
DB_PATH = Path("./salvo.db")

def download_sde():
    """Download the EVE SDE zip file"""
    print("Downloading EVE SDE...")
    if SDE_DIR.exists():
        print("SDE directory already exists, skipping download")
        return

    response = requests.get(SDE_URL, stream=True)
    zip_path = Path("./sde.zip")

    total_size = int(response.headers.get('content-length', 0))
    downloaded = 0

    with open(zip_path, 'wb') as f:
        for chunk in response.iter_content(chunk_size=8192):
            f.write(chunk)
            downloaded += len(chunk)
            if total_size > 0:
                percent = (downloaded / total_size) * 100
                print(f"\r  Progress: {percent:.1f}%", end='')

    print("\nExtracting SDE...")
    with zipfile.ZipFile(zip_path, 'r') as zip_ref:
        zip_ref.extractall(SDE_DIR)

    zip_path.unlink()
    print("SDE downloaded and extracted")

def load_type_materials():
    """Load reprocessing materials from typeMaterials.yaml"""
    print("Loading reprocessing materials...")
    materials_file = SDE_DIR / "fsd" / "typeMaterials.yaml"

    if not materials_file.exists():
        print(f"Materials file not found at {materials_file}")
        return {}

    with open(materials_file, 'r', encoding='utf-8') as f:
        materials_data = yaml.safe_load(f)

    # materials_data format: {typeID: {materials: [{materialTypeID: X, quantity: Y}]}}
    reprocessing_map = {}
    relevant_type_ids = set()

    for type_id, data in materials_data.items():
        if isinstance(data, dict) and 'materials' in data:
            materials = data['materials']
            if materials and isinstance(materials, list):
                relevant_type_ids.add(type_id)
                reprocessing_map[type_id] = materials

                # Also track material type IDs
                for material in materials:
                    mat_type_id = material.get('materialTypeID')
                    if mat_type_id:
                        relevant_type_ids.add(mat_type_id)

    print(f"Found {len(reprocessing_map)} items with reprocessing data")
    print(f"Total relevant type IDs: {len(relevant_type_ids)}")
    return reprocessing_map, relevant_type_ids

def load_types():
    """Load type information from types.yaml"""
    print("Loading type information...")
    types_file = SDE_DIR / "fsd" / "types.yaml"

    if not types_file.exists():
        print(f"Types file not found at {types_file}")
        return {}

    with open(types_file, 'r', encoding='utf-8') as f:
        types_data = yaml.safe_load(f)

    type_info = {}
    for type_id, data in types_data.items():
        if isinstance(data, dict):
            name_data = data.get('name', {})
            name = name_data.get('en', f'Unknown_{type_id}') if isinstance(name_data, dict) else str(name_data)

            type_info[type_id] = {
                'name': name,
                'groupID': data.get('groupID', 0),
                'published': data.get('published', False)
            }

    print(f"Loaded {len(type_info)} types")
    return type_info

def load_blueprints():
    """Load blueprint definitions from blueprints.yaml"""
    print("Loading blueprint definitions...")
    blueprints_file = SDE_DIR / "fsd" / "blueprints.yaml"

    if not blueprints_file.exists():
        print(f"Blueprints file not found at {blueprints_file}")
        return {}

    with open(blueprints_file, 'r', encoding='utf-8') as f:
        blueprints_data = yaml.safe_load(f)

    # blueprints_data format: {blueprintTypeID: {activities: {manufacturing: {...}}}}
    blueprints_map = {}
    relevant_type_ids = set()

    for blueprint_type_id, blueprint_data in blueprints_data.items():
        if not isinstance(blueprint_data, dict):
            continue

        activities = blueprint_data.get('activities', {})
        manufacturing = activities.get('manufacturing', {})

        if not manufacturing:
            continue

        # Get product and materials for manufacturing
        products = manufacturing.get('products', [])
        materials = manufacturing.get('materials', [])
        time = manufacturing.get('time', 0)

        if not products or not materials:
            continue

        # Use first product (most blueprints have only one)
        product = products[0]
        product_type_id = product.get('typeID')

        if product_type_id:
            blueprints_map[blueprint_type_id] = {
                'product_type_id': product_type_id,
                'manufacturing_time': time,
                'materials': materials
            }

            # Track all relevant type IDs
            relevant_type_ids.add(blueprint_type_id)
            relevant_type_ids.add(product_type_id)

            for material in materials:
                mat_type_id = material.get('typeID')
                if mat_type_id:
                    relevant_type_ids.add(mat_type_id)

    print(f"Found {len(blueprints_map)} blueprints with manufacturing data")
    print(f"Total relevant blueprint type IDs: {len(relevant_type_ids)}")
    return blueprints_map, relevant_type_ids

def import_to_database():
    """Import types and reprocessing data into SQLite database"""
    print(f"\nConnecting to database: {DB_PATH}")

    if not DB_PATH.exists():
        print("Database does not exist. Please run the backend first to create it.")
        return

    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()

    # Load all data
    reprocessing_map, reprocessing_type_ids = load_type_materials()
    blueprints_map, blueprint_type_ids = load_blueprints()
    type_info = load_types()

    # Combine all relevant type IDs
    all_relevant_type_ids = reprocessing_type_ids | blueprint_type_ids

    print(f"\nImporting {len(all_relevant_type_ids)} types...")
    imported_count = 0
    reprocessing_count = 0
    blueprint_count = 0
    blueprint_material_count = 0
    skipped_count = 0

    for type_id in all_relevant_type_ids:
        info = type_info.get(type_id)
        if not info:
            skipped_count += 1
            continue

        name = info['name']
        group_id = info['groupID']

        # Default to category 6 (Ship and Module Modifications)
        # This is a simplification - proper category mapping would require group->category lookup
        category_id = 6

        # Import the type
        try:
            cursor.execute("""
                INSERT OR IGNORE INTO eve_types (type_id, name, description, group_id, category_id)
                VALUES (?, ?, ?, ?, ?)
            """, (type_id, name, '', group_id, category_id))

            if cursor.rowcount > 0:
                imported_count += 1

            # Import reprocessing materials if this type can be reprocessed
            if type_id in reprocessing_map:
                for material in reprocessing_map[type_id]:
                    material_type_id = material.get('materialTypeID')
                    quantity = material.get('quantity', 0)

                    if material_type_id and quantity > 0:
                        cursor.execute("""
                            INSERT OR IGNORE INTO material_reprocessing (source_type_id, material_type_id, quantity)
                            VALUES (?, ?, ?)
                        """, (type_id, material_type_id, quantity))

                        if cursor.rowcount > 0:
                            reprocessing_count += 1

            # Import blueprint data if this is a blueprint
            if type_id in blueprints_map:
                blueprint = blueprints_map[type_id]
                product_type_id = blueprint['product_type_id']
                manufacturing_time = blueprint['manufacturing_time']

                cursor.execute("""
                    INSERT OR IGNORE INTO blueprints (blueprint_type_id, product_type_id, manufacturing_time)
                    VALUES (?, ?, ?)
                """, (type_id, product_type_id, manufacturing_time))

                if cursor.rowcount > 0:
                    blueprint_count += 1

                # Import blueprint materials
                for material in blueprint['materials']:
                    material_type_id = material.get('typeID')
                    quantity = material.get('quantity', 0)

                    if material_type_id and quantity > 0:
                        cursor.execute("""
                            INSERT OR IGNORE INTO blueprint_materials (blueprint_type_id, material_type_id, quantity)
                            VALUES (?, ?, ?)
                        """, (type_id, material_type_id, quantity))

                        if cursor.rowcount > 0:
                            blueprint_material_count += 1

            # Commit every 1000 items
            if (imported_count + skipped_count) % 1000 == 0:
                conn.commit()
                print(f"  Progress: {imported_count} types, {reprocessing_count} reprocessing, {blueprint_count} blueprints...")

        except Exception as e:
            print(f"Error importing type {type_id} ({name}): {e}")

    conn.commit()
    conn.close()

    print(f"\nImport complete!")
    print(f"  Types imported: {imported_count}")
    print(f"  Reprocessing entries: {reprocessing_count}")
    print(f"  Blueprints imported: {blueprint_count}")
    print(f"  Blueprint materials: {blueprint_material_count}")
    print(f"  Types skipped (no info): {skipped_count}")

def main():
    print("EVE SDE Importer for Salvo")
    print("=" * 50)

    # Check if database exists
    if not DB_PATH.exists():
        print(f"Error: Database not found at {DB_PATH}")
        print("Please run the backend server first to create the database.")
        return

    # Download SDE if needed
    download_sde()

    # Import data
    import_to_database()

    print("\nDone! Restart the backend to use the new data.")

if __name__ == "__main__":
    main()
