#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"

curl -s -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.doe@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "password": "password123"
  }' | jq .
