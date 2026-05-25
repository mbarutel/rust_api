# API Scripts

Curl scripts for interacting with the Conference Services API. All scripts default to `http://localhost:3000` and can be overridden via environment variables.

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `BASE_URL` | `http://localhost:3000` | API base URL |
| `TOKEN` | *(empty)* | JWT bearer token (required by authenticated endpoints) |

### Getting a token

```bash
# Register a user
./register.sh

# Log in and capture the token
TOKEN=$(./login.sh | jq -r '.token')
export TOKEN
```

---

## Auth

### `register.sh`
Register a new user account.
```bash
./register.sh
```

### `login.sh`
Log in and receive a JWT token.
```bash
./login.sh
```

---

## Users

### `list_users.sh [page] [per_page]`
List all users (paginated).
```bash
./list_users.sh
./list_users.sh 2 20
```

### `get_user.sh <user_id>`
Get a single user by ID.
```bash
./get_user.sh 1
```

### `update_user.sh <user_id>`
Update a user's details.
```bash
./update_user.sh 1
```

### `delete_user.sh <user_id>`
Delete a user by ID.
```bash
./delete_user.sh 1
```

---

## Venues

### `create_venues.sh`
Create three sample venues.
```bash
TOKEN=$TOKEN ./create_venues.sh
```

### `list_venues.sh [page] [per_page]`
List all venues (paginated).
```bash
./list_venues.sh
./list_venues.sh 1 5
```

### `get_venue.sh <venue_id>`
Get a single venue by ID.
```bash
./get_venue.sh 1
```

### `update_venue.sh <venue_id>`
Update a venue's details.
```bash
./update_venue.sh 1
```

### `delete_venue.sh <venue_id>`
Delete a venue by ID.
```bash
./delete_venue.sh 1
```

---

## Conferences

### `create_conferences.sh`
Create three sample conferences.
```bash
TOKEN=$TOKEN ./create_conferences.sh
```

### `list_conferences.sh [page] [per_page]`
List all conferences (paginated).
```bash
./list_conferences.sh
./list_conferences.sh 1 5
```

### `get_conference.sh <conference_id>`
Get a single conference by ID.
```bash
./get_conference.sh 1
```

### `update_conference.sh <conference_id>`
Update a conference's details.
```bash
./update_conference.sh 1
```

### `delete_conference.sh <conference_id>`
Delete a conference by ID.
```bash
./delete_conference.sh 1
```

### `publish_conference.sh <conference_id>`
Publish a conference (make it publicly visible).
```bash
./publish_conference.sh 1
```

### `unpublish_conference.sh <conference_id>`
Unpublish a conference.
```bash
./unpublish_conference.sh 1
```

### `generate_price_tiers.sh <conference_id>`
Generate suggested price tiers for a conference.
```bash
./generate_price_tiers.sh 1
```

### `get_registration_form.sh <conference_id>`
Get the delegate registration form for a conference (no auth required).
```bash
./get_registration_form.sh 1
```

### `register_delegate.sh <conference_id>`
Register one or more delegates for a conference.
```bash
./register_delegate.sh 1
```

---

## Activities

### `list_activities.sh [page] [per_page]`
List all activities (paginated).
```bash
./list_activities.sh
./list_activities.sh 1 20
```

### `get_activity.sh <activity_id>`
Get a single activity by ID.
```bash
./get_activity.sh 1
```

### `create_activity.sh <conference_id>`
Create an activity for a conference.
```bash
./create_activity.sh 1
```

### `update_activity.sh <activity_id>`
Update an activity's details.
```bash
./update_activity.sh 1
```

### `delete_activity.sh <activity_id>`
Delete an activity by ID.
```bash
./delete_activity.sh 1
```

### `list_conference_activities.sh <conference_id>`
List all activities for a specific conference.
```bash
./list_conference_activities.sh 1
```

### `list_activity_bookings.sh <activity_id>`
List all bookings for an activity.
```bash
./list_activity_bookings.sh 1
```

### `book_activity.sh <activity_id> <participant_id>`
Book an activity for a participant.
```bash
./book_activity.sh 1 5
```

### `confirm_activity_booking.sh <activity_id> <participant_id>`
Confirm a participant's activity booking.
```bash
./confirm_activity_booking.sh 1 5
```

### `cancel_activity_booking.sh <activity_id> <participant_id>`
Cancel a participant's activity booking.
```bash
./cancel_activity_booking.sh 1 5
```

### `list_participant_activity_bookings.sh <participant_id>`
List all activity bookings for a participant.
```bash
./list_participant_activity_bookings.sh 5
```

---

## Masterclasses

### `list_masterclasses.sh [page] [per_page]`
List all masterclasses (paginated).
```bash
./list_masterclasses.sh
./list_masterclasses.sh 1 20
```

### `get_masterclass.sh <masterclass_id>`
Get a single masterclass by ID.
```bash
./get_masterclass.sh 1
```

### `create_masterclass.sh <conference_id>`
Create a masterclass for a conference.
```bash
./create_masterclass.sh 1
```

### `update_masterclass.sh <masterclass_id>`
Update a masterclass's details.
```bash
./update_masterclass.sh 1
```

### `delete_masterclass.sh <masterclass_id>`
Delete a masterclass by ID.
```bash
./delete_masterclass.sh 1
```

### `list_conference_masterclasses.sh <conference_id>`
List all masterclasses for a specific conference.
```bash
./list_conference_masterclasses.sh 1
```

### `list_masterclass_instructors.sh <masterclass_id>`
List all instructors for a masterclass.
```bash
./list_masterclass_instructors.sh 1
```

### `add_masterclass_instructor.sh <masterclass_id> <participant_id>`
Add a participant as an instructor for a masterclass.
```bash
./add_masterclass_instructor.sh 1 5
```

### `remove_masterclass_instructor.sh <masterclass_id> <participant_id>`
Remove an instructor from a masterclass.
```bash
./remove_masterclass_instructor.sh 1 5
```

### `list_masterclass_bookings.sh <masterclass_id>`
List all bookings for a masterclass.
```bash
./list_masterclass_bookings.sh 1
```

### `book_masterclass.sh <masterclass_id> <participant_id>`
Book a masterclass for a participant.
```bash
./book_masterclass.sh 1 5
```

### `confirm_masterclass_booking.sh <masterclass_id> <participant_id>`
Confirm a participant's masterclass booking.
```bash
./confirm_masterclass_booking.sh 1 5
```

### `cancel_masterclass_booking.sh <masterclass_id> <participant_id>`
Cancel a participant's masterclass booking.
```bash
./cancel_masterclass_booking.sh 1 5
```

### `list_participant_masterclass_bookings.sh <participant_id>`
List all masterclass bookings for a participant.
```bash
./list_participant_masterclass_bookings.sh 5
```

---

## Group Discounts

All group discount endpoints require a `TOKEN`.

### `list_group_discounts.sh [page] [per_page]`
List all group discounts (paginated).
```bash
./list_group_discounts.sh
./list_group_discounts.sh 1 20
```

### `get_group_discount.sh <group_discount_id>`
Get a single group discount by ID.
```bash
./get_group_discount.sh 1
```

### `create_group_discount.sh`
Create a new group discount.
```bash
./create_group_discount.sh
```

### `update_group_discount.sh <group_discount_id>`
Update a group discount's details.
```bash
./update_group_discount.sh 1
```

### `delete_group_discount.sh <group_discount_id>`
Delete a group discount by ID.
```bash
./delete_group_discount.sh 1
```

---

## Clients

### `list_clients.sh [page] [per_page]`
List all clients (paginated).
```bash
./list_clients.sh
./list_clients.sh 1 20
```

### `get_client.sh <client_id>`
Get a single client by ID.
```bash
./get_client.sh 1
```

### `create_client.sh`
Create a new client.
```bash
./create_client.sh
```

### `update_client.sh <client_id>`
Update a client's details.
```bash
./update_client.sh 1
```

### `delete_client.sh <client_id>`
Delete a client by ID.
```bash
./delete_client.sh 1
```

---

## Organizations

### `list_organizations.sh [page] [per_page]`
List all organizations (paginated).
```bash
./list_organizations.sh
./list_organizations.sh 1 20
```

### `get_organization.sh <organization_id>`
Get a single organization by ID.
```bash
./get_organization.sh 1
```

### `create_organization.sh`
Create a new organization.
```bash
./create_organization.sh
```

### `update_organization.sh <organization_id>`
Update an organization's details.
```bash
./update_organization.sh 1
```

### `delete_organization.sh <organization_id>`
Delete an organization by ID.
```bash
./delete_organization.sh 1
```

---

## Participants

### `list_participants.sh [page] [per_page]`
List all participants (paginated).
```bash
./list_participants.sh
./list_participants.sh 1 20
```

### `get_participant.sh <participant_id>`
Get a single participant by ID.
```bash
./get_participant.sh 1
```

### `create_participant.sh <registration_id> <client_id>`
Create a participant linked to a registration and client.
```bash
./create_participant.sh 1 3
```

### `update_participant.sh <participant_id>`
Update a participant's details.
```bash
./update_participant.sh 1
```

### `delete_participant.sh <participant_id>`
Delete a participant by ID.
```bash
./delete_participant.sh 1
```

### `list_registration_participants.sh <registration_id>`
List all participants for a registration.
```bash
./list_registration_participants.sh 1
```

---

## Registrations

### `list_registrations.sh [page] [per_page]`
List all registrations (paginated).
```bash
./list_registrations.sh
./list_registrations.sh 1 20
```

### `get_registration.sh <registration_id>`
Get a single registration by ID.
```bash
./get_registration.sh 1
```

### `create_registration.sh <conference_id>`
Create a new registration for a conference.
```bash
./create_registration.sh 1
```

### `update_registration.sh <registration_id>`
Update a registration's details.
```bash
./update_registration.sh 1
```

### `delete_registration.sh <registration_id>`
Delete a registration by ID.
```bash
./delete_registration.sh 1
```

### `transition_registration_status.sh <registration_id> <status>`
Transition a registration to a new status.
```bash
./transition_registration_status.sh 1 confirmed
./transition_registration_status.sh 1 cancelled
```

### `record_registration_payment.sh <registration_id> <amount>`
Record a payment against a registration.
```bash
./record_registration_payment.sh 1 299.00
```
