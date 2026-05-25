#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
CONFERENCE_ID="${1:?Usage: $0 <conference_id>}"

curl -s -X POST "$BASE_URL/api/registrations" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "conference_id": '"$CONFERENCE_ID"',
    "created_by_id": null,
    "cost": "299.00",
    "discount_code": null,
    "discount_amount": null,
    "notes_internal": null
  }' | jq .
