#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
REGISTRATION_ID="${1:?Usage: $0 <registration_id> <status>}"
STATUS="${2:?Usage: $0 <registration_id> <status>}"

curl -s -X PUT "$BASE_URL/api/registrations/$REGISTRATION_ID/status" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"status": "'"$STATUS"'"}' | jq .
