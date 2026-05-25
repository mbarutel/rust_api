#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
CONFERENCE_ID="${1:?Usage: $0 <conference_id>}"

curl -s -X PUT "$BASE_URL/api/conferences/$CONFERENCE_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "RustConf 2025 (Updated)",
    "description": "Updated description for the annual Rust conference",
    "venue_id": 2,
    "group_discount": 1,
    "start_date": "2025-11-01T09:00:00",
    "end_date": "2025-11-03T17:00:00"
  }' | jq .
