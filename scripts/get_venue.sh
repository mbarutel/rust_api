#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
VENUE_ID="${1:?Usage: $0 <venue_id>}"

curl -s -X GET "$BASE_URL/api/venues/$VENUE_ID" \
  -H "Authorization: Bearer $TOKEN" | jq .
