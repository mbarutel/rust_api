#!/usr/bin/env bash
BASE_URL="${BASE_URL:-http://localhost:3000}"
CONFERENCE_ID="${1:?Usage: $0 <conference_id>}"

curl -s -X POST "$BASE_URL/api/conferences/$CONFERENCE_ID/register/delegate" \
  -H "Content-Type: application/json" \
  -d '{
    "conference_id": '"$CONFERENCE_ID"',
    "price_tier": {
      "price": "299.00",
      "deadline": "2026-06-01T00:00:00Z"
    },
    "discount_code": null,
    "delegates": [
      {
        "first_name": "John",
        "last_name": "Doe",
        "job_title": "Software Engineer",
        "organization_name": "Acme Corp",
        "email": "john.doe@example.com",
        "dietary_requirements": "none",
        "networking_dinner": true,
        "masterclass_selection": null,
        "accomodation_nights": 2
      }
    ],
    "referrer": "Colleague"
  }' | jq .
