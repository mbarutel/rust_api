#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
MASTERCLASS_ID="${1:?Usage: $0 <masterclass_id> <participant_id>}"
PARTICIPANT_ID="${2:?Usage: $0 <masterclass_id> <participant_id>}"

curl -s -X DELETE "$BASE_URL/api/masterclasses/$MASTERCLASS_ID/instructors/$PARTICIPANT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -w "\nHTTP Status: %{http_code}\n"
