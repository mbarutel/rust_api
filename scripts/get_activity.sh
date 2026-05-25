#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
ACTIVITY_ID="${1:?Usage: $0 <activity_id>}"

curl -s -X GET "$BASE_URL/api/activities/$ACTIVITY_ID" | jq .
