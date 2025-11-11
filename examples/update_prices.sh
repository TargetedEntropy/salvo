#!/bin/bash

# Script to update market prices from ESI API

BASE_URL="http://127.0.0.1:3000"

echo "=== Updating Market Prices from ESI API ==="
echo ""

# Update prices for common salvage and materials
echo "Updating prices for salvage items and materials..."
curl -s -X POST $BASE_URL/api/market/update \
  -H "Content-Type: application/json" \
  -d '{
    "type_ids": [
      25595, 25596, 25597, 25604, 25605, 25606,
      34, 35, 36, 37, 38, 39, 40,
      26898, 26904, 26902
    ]
  }' | jq '.'

echo ""
echo "=== Price Update Complete ==="
