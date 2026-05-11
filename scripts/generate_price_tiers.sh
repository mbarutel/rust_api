#!/bin/sh
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
CONFERENCE_ID="${1:?Usage: $0 <conference_id>}"

curl -s -X GET "$BASE_URL/api/conferences/$CONFERENCE_ID/generate-price-tiers" \
  -H "Authorization: Bearer $TOKEN" | jq .
