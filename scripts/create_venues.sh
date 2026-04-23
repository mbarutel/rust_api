#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
TOKEN="${TOKEN:-}"

echo "Creating venue 1..."
curl -s -X POST "$BASE_URL/api/venues" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Grand Convention Center",
    "address_line1": "123 Main Street",
    "address_line2": "Suite 100",
    "city": "New York",
    "state_region": "NY",
    "postal_code": "10001",
    "country": "US",
    "notes": "Large main hall with capacity for 2000 attendees"
  }' | jq .

echo "Creating venue 2..."
curl -s -X POST "$BASE_URL/api/venues" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Riverside Conference Hall",
    "address_line1": "456 River Road",
    "city": "Chicago",
    "state_region": "IL",
    "postal_code": "60601",
    "country": "US",
    "notes": "Overlooking the river, ideal for mid-size events"
  }' | jq .

echo "Creating venue 3..."
curl -s -X POST "$BASE_URL/api/venues" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Sunset Expo Center",
    "address_line1": "789 Pacific Ave",
    "city": "Los Angeles",
    "state_region": "CA",
    "postal_code": "90001",
    "country": "US",
    "notes": "Outdoor pavilion and indoor exhibition space"
  }' | jq .
