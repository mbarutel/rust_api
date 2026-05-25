#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"
CLIENT_ID="${1:?Usage: $0 <client_id>}"

curl -s -X DELETE "$BASE_URL/api/clients/$CLIENT_ID" \
  -H "Authorization: Bearer $TOKEN" \
  -w "\nHTTP Status: %{http_code}\n"
