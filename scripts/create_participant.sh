#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
REGISTRATION_ID="${1:?Usage: $0 <registration_id> <client_id>}"
CLIENT_ID="${2:?Usage: $0 <registration_id> <client_id>}"

curl -s -X POST "$BASE_URL/api/participants" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "registration_id": '"$REGISTRATION_ID"',
    "client_id": '"$CLIENT_ID"',
    "role": "attendee",
    "dietary_requirements": "none",
    "accessibility_needs": null
  }' | jq .
