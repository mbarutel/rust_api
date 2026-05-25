#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
ACTIVITY_ID="${1:?Usage: $0 <activity_id>}"

curl -s -X DELETE "$BASE_URL/api/activities/$ACTIVITY_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -w "\nHTTP Status: %{http_code}\n"
