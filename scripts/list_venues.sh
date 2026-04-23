#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
PAGE="${1:-1}"
PER_PAGE="${2:-10}"

curl -s -X GET "$BASE_URL/api/venues?page=$PAGE&per_page=$PER_PAGE" \
  -H "Authorization: Bearer $TOKEN" | jq .
