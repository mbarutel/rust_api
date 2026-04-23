#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"

echo "Creating conference 1..."
curl -s -X POST "$BASE_URL/api/conferences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "code": "RUST",
    "name": "RustConf 2025",
    "poster_url": "https://example.com/rustconf.png",
    "description": "The annual Rust programming language conference",
    "start_date": "2025-09-15T09:00:00",
    "end_date": "2025-09-16T18:00:00",
    "venue_id": 1
  }' | jq .

echo "Creating conference 2..."
curl -s -X POST "$BASE_URL/api/conferences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "code": "OSCP",
    "name": "Open Source Conference 2025",
    "description": "Celebrating open source software",
    "start_date": "2025-10-01T09:00:00",
    "end_date": "2025-10-03T17:00:00",
    "venue_id": 2
  }' | jq .

echo "Creating conference 3..."
curl -s -X POST "$BASE_URL/api/conferences" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "code": "CLOD",
    "name": "Cloud Summit 2025",
    "description": "Cloud infrastructure and DevOps",
    "start_date": "2025-11-20T08:00:00",
    "end_date": "2025-11-21T17:00:00"
  }' | jq .
