#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
CONFERENCE_ID="${1:?Usage: $0 <conference_id>}"

curl -s -X POST "$BASE_URL/api/activities" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "conference_id": '"$CONFERENCE_ID"',
    "name": "Networking Session",
    "description": "Open networking for all attendees",
    "start_at": "2025-09-15T17:00:00",
    "end_at": "2025-09-15T19:00:00",
    "venue_id": null,
    "provider_url": null,
    "capacity": 100
  }' | jq .
