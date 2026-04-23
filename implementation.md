# Implementation Plan: New Domain Models

## Overview

12 new tables split into 6 phases by dependency order. Each phase must complete before the next begins.

```
organizations
└── clients
    └── submissions ──────────── conferences (existing)
        └── participants
            ├── speakers
            ├── sponsors
            ├── exhibitors
            ├── activity_bookings ──── activities ─── conferences, venues (existing)
            ├── masterclass_bookings ─ masterclasses ─ conferences, venues (existing)
            └── masterclass_instructors
```

---

## Per-model file checklist

Every model (except junction tables) requires the same 9 files:

| # | File | Layer |
|---|------|-------|
| 1 | `domain/models/{model}.rs` | Domain |
| 2 | `application/entity/{model}_entity.rs` | Application |
| 3 | `application/dto/{model}_dto.rs` | Application |
| 4 | `application/repository/{model}_repository.rs` | Application |
| 5 | `application/service/{model}_service.rs` | Application |
| 6 | `infrastructure/database/repository/{model}_repository.rs` | Infrastructure |
| 7 | `infrastructure/service/{model}_service.rs` | Infrastructure |
| 8 | `presentation/handler/{model}_handler.rs` | Presentation |
| 9 | mod.rs updates + state.rs + lib.rs | Wiring |

---

## Phase 1 — Organization

No dependencies. Simplest model, good starting point to validate the pattern.

**Domain model** — straightforward fields, `website`/`phone` optional.

**Entity** — no `published` column, so no `is_published()` helper needed.

**DTO considerations:**
- `CreateOrganizationRequest`: `name`, `billing_email` required; rest optional
- `UpdateOrganizationRequest`: all fields optional
- `OrganizationResponse`: all fields

**Service** — standard CRUD, no special business logic.

**Handler** — standard 5 routes (`GET /api/organizations`, `POST`, `GET /:id`, `PUT /:id`, `DELETE /:id`).

---

## Phase 2 — Client

Depends on: `organizations`

**Domain model** — include `organization_id: Option<u64>` (FK, not the full `Organization` — clients are usually fetched standalone).

**DTO considerations:**
- `CreateClientRequest`: `first_name`, `last_name`, `email` required; `organization_id` optional
- `UpdateClientRequest`: all fields optional
- Email uniqueness is a business rule — the service `create` must check it (same pattern as `UserServiceImpl`)

**Repository trait** — extend the base `Repository<ClientEntity>` with `find_by_email` (mirrors `UserRepository`):
```rust
pub trait ClientRepository: Repository<ClientEntity> {
    async fn find_by_email(&self, email: &str) -> Result<ClientEntity, DomainError>;
    async fn email_exists(&self, email: &str) -> Result<bool, DomainError>;
}
```

---

## Phase 3 — Submission + Participant

Depends on: `conferences`, `clients`

These two are tightly coupled and should be implemented together.

**Submission domain model** — the `status` field should be a Rust enum:
```rust
pub enum SubmissionStatus {
    Draft,
    Submitted,
    UnderReview,
    Accepted,
    Waitlisted,
    Rejected,
    Withdrawn,
}
```

Define this in `domain/models/submission.rs`. The entity stores it as a `String` (sqlx maps MySQL ENUMs to `String`).

**Submission service** — add a dedicated `transition_status` method rather than allowing free `update` on status. The service enforces valid transitions (e.g. `Draft → Submitted` is valid, `Accepted → Draft` is not).

**Participant domain model** — `participant_role` is also an enum:
```rust
pub enum ParticipantRole {
    Delegate,
    Speaker,
    Sponsor,
    Exhibitor,
}
```

**Participant entity** — no `created_at`/`updated_at` columns in the schema. The entity struct will not have those fields. This breaks the base `Repository<T>` pattern for `create`/`update` since you can't set timestamps. Either:
- Add the timestamp columns in a migration (recommended — consistent with all other tables)
- Or keep it as-is and accept that participants have no audit trail

**Participant repository** — the participant route will typically be nested under a submission: `GET /api/submissions/:id/participants`. Add a domain-specific method:
```rust
pub trait ParticipantRepository: Repository<ParticipantEntity> {
    async fn find_by_submission(&self, submission_id: u64) -> Result<Vec<ParticipantEntity>, DomainError>;
}
```

---

## Phase 4 — Speaker, Sponsor, Exhibitor

Depends on: `participants`

These are 1:1 detail records attached to a participant. They follow the same pattern but have no `find_all` / pagination use case of their own — they are always fetched via their participant.

**Repository trait** — extend base with a lookup by participant:
```rust
pub trait SpeakerRepository: Repository<SpeakerEntity> {
    async fn find_by_participant(&self, participant_id: u64) -> Result<SpeakerEntity, DomainError>;
}
```

**Service** — the service should also expose `find_by_participant_id` as the primary access pattern.

**`speakers` entity** — no `created_at`/`updated_at` in the schema. Same decision point as participants — recommend adding them in a migration.

**Handler routes** — nest under participants:
- `POST /api/participants/:id/speaker`
- `GET /api/participants/:id/speaker`
- `PUT /api/participants/:id/speaker`
- `DELETE /api/participants/:id/speaker`

Same pattern for sponsor and exhibitor.

---

## Phase 5 — Activity + Masterclass

Depends on: `conferences`, `venues`

These are independent of the submission flow. Implement them in parallel with Phase 4.

**Activity domain model** — uses `start_at`/`end_at` (not `start_date`/`end_date` like Conference — keep that distinction in the struct field names).

**Activity repository** — add a method to fetch all activities for a conference:
```rust
pub trait ActivityRepository: Repository<ActivityEntity> {
    async fn find_by_conference(&self, conference_id: u64) -> Result<Vec<ActivityEntity>, DomainError>;
}
```

**Masterclass** — same shape as Activity. The `venue_id` is optional (masterclass can be in the conference venue itself).

**Masterclass instructors** — this is a junction table with a composite PK (`masterclass_id`, `participant_id`) and no surrogate `id`. It does **not** fit the base `Repository<T>` trait (which assumes `find_by_id(u64)`). Model it differently:

```rust
pub trait MasterclassInstructorRepository: Send + Sync {
    async fn add(&self, masterclass_id: u64, participant_id: u64, is_lead: bool) -> Result<(), DomainError>;
    async fn remove(&self, masterclass_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn find_by_masterclass(&self, masterclass_id: u64) -> Result<Vec<MasterclassInstructor>, DomainError>;
}
```

Manage this through the `MasterclassService` rather than giving it its own service.

---

## Phase 6 — Activity Bookings + Masterclass Bookings

Depends on: Phase 4 (participants) + Phase 5 (activities, masterclasses)

Both are junction tables with `status` ENUMs. Same approach as masterclass instructors — custom repository traits rather than the base `Repository<T>`. Manage bookings through their parent service (`ActivityService`, `MasterclassService`) rather than standalone services.

```rust
pub trait ActivityBookingRepository: Send + Sync {
    async fn book(&self, activity_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn cancel(&self, activity_id: u64, participant_id: u64) -> Result<(), DomainError>;
    async fn find_by_activity(&self, activity_id: u64) -> Result<Vec<ActivityBooking>, DomainError>;
    async fn find_by_participant(&self, participant_id: u64) -> Result<Vec<ActivityBooking>, DomainError>;
}
```

---

## Wiring (done once at end of each phase)

For each completed phase, update:

| File | What to add |
|------|-------------|
| `application/entity/mod.rs` | `pub mod {model}_entity;` |
| `application/dto/mod.rs` | `pub mod {model}_dto;` |
| `application/repository/mod.rs` | `pub mod {model}_repository;` |
| `application/service/mod.rs` | `pub mod {model}_service;` |
| `infrastructure/database/repository/mod.rs` | `pub mod {model}_repository;` |
| `infrastructure/service/mod.rs` | `pub mod {model}_service;` |
| `presentation/handler/mod.rs` | `pub mod {model}_handler;` |
| `state.rs` | Add service field + wire in `AppState::init()` |
| `lib.rs` `build_router()` | `.merge({model}_routes())` |
| `presentation/handler/utils.rs` | Add `Mock{Model}Service` to `test_state` |

---

## Recommended order

| Phase | Model | Reason |
|-------|-------|--------|
| 1 | Organization | No deps, validates the workflow |
| 2 | Client | Depends only on Org |
| 3a | Submission | Depends on Conference + Client |
| 3b | Participant | Depends on Submission + Client |
| 4a | Speaker | Simplest detail record |
| 4b | Sponsor | Same shape as Speaker |
| 4c | Exhibitor | Same shape as Speaker |
| 5a | Activity | Independent of submission flow |
| 5b | Masterclass | Same shape as Activity |
| 5c | MasterclassInstructor | Via Masterclass service |
| 6a | ActivityBooking | Via Activity service |
| 6b | MasterclassBooking | Via Masterclass service |
