#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
REGISTRATION_ID="${1:?Usage: $0 <registration_id>}"

curl -s -X DELETE "$BASE_URL/api/registrations/$REGISTRATION_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -w "\nHTTP Status: %{http_code}\n"
