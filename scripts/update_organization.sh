#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
ORGANIZATION_ID="${1:?Usage: $0 <organization_id>}"

curl -s -X PUT "$BASE_URL/api/organizations/$ORGANIZATION_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Acme Corp (Updated)",
    "website": "https://acme-updated.example.com",
    "billing_email": "accounts@acme.example.com"
  }' | jq .
