#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
CONFERENCE_ID="${1:?Usage: $0 <conference_id>}"

curl -s -X GET "$BASE_URL/api/conferences/$CONFERENCE_ID/registration-form" | jq .
