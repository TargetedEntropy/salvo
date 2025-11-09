#!/bin/bash

# Test script for Salvo API

BASE_URL="http://127.0.0.1:3000"

echo "=== Salvo API Test Script ==="
echo ""

# Health check
echo "1. Health Check"
curl -s $BASE_URL/health
echo -e "\n"

# Root endpoint
echo "2. API Info"
curl -s $BASE_URL/
echo -e "\n\n"

# Test Case 1: Small salvage haul
echo "3. Small Salvage Haul (10x Tripped Power Circuit, 8x Charred Micro Circuit)"
curl -s -X POST $BASE_URL/api/salvage/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "salvage_items": [
      {"name": "Tripped Power Circuit", "quantity": 10},
      {"name": "Charred Micro Circuit", "quantity": 8}
    ],
    "reprocessing_efficiency": 0.5
  }' | jq '.buildable_items[] | {
    name: .product_name,
    can_build,
    match_pct: .match_percentage,
    profit: .estimated_profit,
    price: .product_price
  }'
echo -e "\n"

# Test Case 2: Large salvage haul
echo "4. Large Salvage Haul (100x each)"
curl -s -X POST $BASE_URL/api/salvage/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "salvage_items": [
      {"name": "Tripped Power Circuit", "quantity": 100},
      {"name": "Charred Micro Circuit", "quantity": 100},
      {"name": "Fried Interface Circuit", "quantity": 100},
      {"name": "Contaminated Nanite Compound", "quantity": 50},
      {"name": "Armor Plates", "quantity": 30}
    ],
    "reprocessing_efficiency": 0.6
  }' | jq '{
    total_material_value,
    material_count: (.materials | length),
    buildable_count: (.buildable_items | length),
    fully_buildable: [.buildable_items[] | select(.can_build == true) | .product_name]
  }'
echo -e "\n"

# Test Case 3: Perfect reprocessing skills
echo "5. Perfect Reprocessing Skills (69.575% efficiency)"
curl -s -X POST $BASE_URL/api/salvage/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "salvage_items": [
      {"name": "Tripped Power Circuit", "quantity": 50},
      {"name": "Charred Micro Circuit", "quantity": 50}
    ],
    "reprocessing_efficiency": 0.69575
  }' | jq '{
    efficiency_used: .reprocessing_efficiency_used,
    total_materials_value: .total_material_value,
    best_profit_item: (.buildable_items | max_by(.estimated_profit) | {
      name: .product_name,
      profit: .estimated_profit,
      can_build
    })
  }'
echo -e "\n"

# Test Case 4: Material breakdown
echo "6. Material Breakdown Detail"
curl -s -X POST $BASE_URL/api/salvage/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "salvage_items": [
      {"name": "Tripped Power Circuit", "quantity": 20}
    ],
    "reprocessing_efficiency": 0.5
  }' | jq '.materials[] | {
    name,
    quantity,
    unit_price,
    total_value
  }'
echo -e "\n"

echo "=== Test Complete ==="
