#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
ORGANIZATION_ID="${1:?Usage: $0 <organization_id>}"

curl -s -X GET "$BASE_URL/api/organizations/$ORGANIZATION_ID" | jq .
