#!/bin/bash

# Script to backfill ALL market prices from ESI

BASE_URL="http://127.0.0.1:3000"

echo "=== Backfilling ALL Market Prices from ESI ==="
echo ""
echo "This will fetch and store prices for all ~15k+ items from ESI..."
echo "This may take 30-60 seconds."
echo ""

# Call with empty type_ids array to trigger backfill mode
curl -s -X POST $BASE_URL/api/market/update \
  -H "Content-Type: application/json" \
  -d '{"type_ids": []}' | jq '.'

echo ""
echo "=== Backfill Complete ==="
