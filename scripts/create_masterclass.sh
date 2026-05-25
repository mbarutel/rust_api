#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
CONFERENCE_ID="${1:?Usage: $0 <conference_id>}"

curl -s -X POST "$BASE_URL/api/masterclasses" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "conference_id": '"$CONFERENCE_ID"',
    "name": "Advanced Rust Workshop",
    "description": "Deep dive into Rust ownership and async",
    "start_at": "2025-09-15T09:00:00",
    "end_at": "2025-09-15T13:00:00",
    "venue_id": null,
    "capacity": 30
  }' | jq .
