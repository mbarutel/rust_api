#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
ACTIVITY_ID="${1:?Usage: $0 <activity_id> <participant_id>}"
PARTICIPANT_ID="${2:?Usage: $0 <activity_id> <participant_id>}"

curl -s -X POST "$BASE_URL/api/activities/$ACTIVITY_ID/bookings" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"participant_id": '"$PARTICIPANT_ID"'}' \
  -w "\nHTTP Status: %{http_code}\n"
