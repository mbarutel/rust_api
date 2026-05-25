#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
VENUE_ID="${1:?Usage: $0 <venue_id>}"

curl -s -X PUT "$BASE_URL/api/venues/$VENUE_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Updated Convention Center",
    "address_line1": "123 Main Street",
    "address_line2": "Suite 200",
    "city": "New York",
    "state_region": "NY",
    "postal_code": "10001",
    "country": "US",
    "notes": "Updated notes"
  }' | jq .
