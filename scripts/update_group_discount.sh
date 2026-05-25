#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
GROUP_DISCOUNT_ID="${1:?Usage: $0 <group_discount_id>}"

curl -s -X PUT "$BASE_URL/api/group-discounts/$GROUP_DISCOUNT_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Updated Group Discount",
    "min_quantity": 3,
    "free_quantity": 1,
    "valid_until": "2027-06-30T23:59:59"
  }' | jq .
