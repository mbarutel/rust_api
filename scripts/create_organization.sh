#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"

curl -s -X POST "$BASE_URL/api/organizations" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Acme Corp",
    "website": "https://acme.example.com",
    "phone": "+1-555-0100",
    "billing_email": "billing@acme.example.com"
  }' | jq .
