#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
PAGE="${1:-1}"
PER_PAGE="${2:-10}"

curl -s -X GET "$BASE_URL/api/organizations?page=$PAGE&per_page=$PER_PAGE" | jq .
