#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
REGISTRATION_ID="${1:?Usage: $0 <registration_id>}"

curl -s -X PUT "$BASE_URL/api/registrations/$REGISTRATION_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "cost": "349.00",
    "discount_code": null,
    "discount_amount": null,
    "notes_internal": "Updated by admin"
  }' | jq .
