#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
MASTERCLASS_ID="${1:?Usage: $0 <masterclass_id> <participant_id>}"
PARTICIPANT_ID="${2:?Usage: $0 <masterclass_id> <participant_id>}"

curl -s -X POST "$BASE_URL/api/masterclasses/$MASTERCLASS_ID/bookings" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"participant_id": '"$PARTICIPANT_ID"'}' \
  -w "\nHTTP Status: %{http_code}\n"
