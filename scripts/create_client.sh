#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"

curl -s -X POST "$BASE_URL/api/clients" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "first_name": "Alice",
    "last_name": "Smith",
    "email": "alice.smith@example.com",
    "organization_id": null
  }' | jq .
