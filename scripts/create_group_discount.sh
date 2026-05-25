#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"

curl -s -X POST "$BASE_URL/api/group-discounts" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Buy 2 Get 1 Free",
    "code": "BUY2GET1",
    "min_quantity": 2,
    "free_quantity": 1,
    "valid_until": "2026-12-31T23:59:59"
  }' | jq .
