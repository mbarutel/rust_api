#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
PARTICIPANT_ID="${1:?Usage: $0 <participant_id>}"

curl -s -X PUT "$BASE_URL/api/participants/$PARTICIPANT_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "role": "speaker",
    "dietary_requirements": "vegetarian",
    "accessibility_needs": null
  }' | jq .
