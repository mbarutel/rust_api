#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
CLIENT_ID="${1:?Usage: $0 <client_id>}"

curl -s -X GET "$BASE_URL/api/clients/$CLIENT_ID" | jq .
