#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
REGISTRATION_ID="${1:?Usage: $0 <registration_id> <amount>}"
AMOUNT="${2:?Usage: $0 <registration_id> <amount>}"

curl -s -X POST "$BASE_URL/api/registrations/$REGISTRATION_ID/payments" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"amount": "'"$AMOUNT"'"}' | jq .
