#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
ACTIVITY_ID="${1:?Usage: $0 <activity_id>}"

curl -s -X PUT "$BASE_URL/api/activities/$ACTIVITY_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Updated Networking Session",
    "description": "Updated description",
    "capacity": 150
  }' | jq .
