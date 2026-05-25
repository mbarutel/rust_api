#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
MASTERCLASS_ID="${1:?Usage: $0 <masterclass_id>}"

curl -s -X PUT "$BASE_URL/api/masterclasses/$MASTERCLASS_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Updated Rust Workshop",
    "description": "Updated description",
    "capacity": 40
  }' | jq .
