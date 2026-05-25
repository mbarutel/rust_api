#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
ORGANIZATION_ID="${1:?Usage: $0 <organization_id>}"

curl -s -X DELETE "$BASE_URL/api/organizations/$ORGANIZATION_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -w "\nHTTP Status: %{http_code}\n"
