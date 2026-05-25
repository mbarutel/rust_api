#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
REGISTRATION_ID="${1:?Usage: $0 <registration_id>}"

curl -s -X GET "$BASE_URL/api/registrations/$REGISTRATION_ID" | jq .
