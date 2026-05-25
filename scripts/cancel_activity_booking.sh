#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
ACTIVITY_ID="${1:?Usage: $0 <activity_id> <participant_id>}"
PARTICIPANT_ID="${2:?Usage: $0 <activity_id> <participant_id>}"

curl -s -X DELETE "$BASE_URL/api/activities/$ACTIVITY_ID/bookings/$PARTICIPANT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -w "\nHTTP Status: %{http_code}\n"
