#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
CLIENT_ID="${1:?Usage: $0 <client_id>}"

curl -s -X PUT "$BASE_URL/api/clients/$CLIENT_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "first_name": "Alice",
    "last_name": "Johnson",
    "email": "alice.johnson@example.com"
  }' | jq .
