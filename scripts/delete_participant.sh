#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
PARTICIPANT_ID="${1:?Usage: $0 <participant_id>}"

curl -s -X DELETE "$BASE_URL/api/participants/$PARTICIPANT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -w "\nHTTP Status: %{http_code}\n"
