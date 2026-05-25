#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
GROUP_DISCOUNT_ID="${1:?Usage: $0 <group_discount_id>}"

curl -s -X GET "$BASE_URL/api/group-discounts/$GROUP_DISCOUNT_ID" \
  -H "Authorization: Bearer $TOKEN" | jq .
