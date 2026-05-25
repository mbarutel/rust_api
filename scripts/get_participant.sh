#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
PARTICIPANT_ID="${1:?Usage: $0 <participant_id>}"

curl -s -X GET "$BASE_URL/api/participants/$PARTICIPANT_ID" | jq .
