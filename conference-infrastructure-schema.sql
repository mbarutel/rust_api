-- Schema: Event management registrations (MySQL 8+)
-- Engine/charset defaults
SET NAMES utf8mb4 COLLATE utf8mb4_0900_ai_ci;

-- Optional: create & use a database
-- CREATE DATABASE IF NOT EXISTS event_mgmt CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci;
-- USE event_mgmt;

-- ----------------------------
-- Venues
-- ----------------------------
CREATE TABLE IF NOT EXISTS venues (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  name             VARCHAR(255) NOT NULL,
  address_line1    VARCHAR(255),
  address_line2    VARCHAR(255),
  city             VARCHAR(120),
  state_region     VARCHAR(120),
  postal_code      VARCHAR(40),
  country          VARCHAR(120),
  notes            TEXT,
  created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Conferences
-- ----------------------------
CREATE TABLE IF NOT EXISTS conferences (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  code             VARCHAR(64) NOT NULL UNIQUE,
  name             VARCHAR(255) NOT NULL,
  poster_url       VARCHAR(512) NOT NULL,
  description      TEXT NOT NULL,
  start_date       DATETIME NOT NULL,
  end_date         DATETIME NOT NULL,
  venue_id         BIGINT UNSIGNED,
  created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_conferences_venue
    FOREIGN KEY (venue_id) REFERENCES venues(id)
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_conferences_dates ON conferences(start_date, end_date);

-- ----------------------------
-- Organizations (optional B2B grouping)
-- ----------------------------
CREATE TABLE organizations (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  name             VARCHAR(255) NOT NULL,
  website          VARCHAR(255),
  phone            VARCHAR(64),
  billing_email    VARCHAR(255) NOT NULL,
  created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Clients (people)
-- ----------------------------
CREATE TABLE clients (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  organization_id  BIGINT UNSIGNED,
  first_name       VARCHAR(120) NOT NULL,
  last_name        VARCHAR(120) NOT NULL,
  email            VARCHAR(255) NOT NULL,
  phone            VARCHAR(64),
  job_title        VARCHAR(160),
  created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  UNIQUE KEY uq_clients_email (email),
  CONSTRAINT fk_clients_org
    FOREIGN KEY (organization_id) REFERENCES organizations(id)
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_clients_org ON clients(organization_id);

-- ----------------------------
-- Submissions (group applications)
-- ----------------------------
CREATE TABLE submissions (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  conference_id    BIGINT UNSIGNED NOT NULL,
  status           ENUM('draft','submitted','under_review','accepted','waitlisted','rejected','withdrawn')
                     NOT NULL DEFAULT 'draft',
  created_by_id    BIGINT UNSIGNED,
  notes_internal   TEXT,
  created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_submissions_conf
    FOREIGN KEY (conference_id) REFERENCES conferences(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_submissions_created_by
    FOREIGN KEY (created_by_id) REFERENCES clients(id)
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_submissions_conf ON submissions(conference_id);
CREATE INDEX idx_submissions_status ON submissions(status);

-- ----------------------------
-- Submission participants (people/entities in the submission)
-- Default role = delegate
-- ----------------------------
CREATE TABLE participants (
  id                 BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  submission_id      BIGINT UNSIGNED NOT NULL,
  client_id          BIGINT UNSIGNED NOT NULL,
  participant_role   ENUM('delegate','speaker','sponsor','exhibitor')
                       NOT NULL DEFAULT 'delegate',
  -- base fields common to everyone
  dietary_requirements TEXT,
  accessibility_needs  TEXT,
  CONSTRAINT uq_submission_client UNIQUE (submission_id, client_id),
  CONSTRAINT fk_sp_submission
    FOREIGN KEY (submission_id) REFERENCES submissions(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_sp_client
    FOREIGN KEY (client_id) REFERENCES clients(id)
    ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_sp_submission ON participants(submission_id);
CREATE INDEX idx_sp_client ON participants(client_id);
CREATE INDEX idx_sp_role ON participants(participant_role);

-- ----------------------------
-- Speaker details (1:1 with a participant)
-- ----------------------------
CREATE TABLE speakers (
  id                BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  participant_id    BIGINT UNSIGNED NOT NULL,
  talk_title        VARCHAR(255) NOT NULL,
  talk_abstract     TEXT,
  duration_minutes  INT,
  av_requirements   TEXT,
  headshot          VARCHAR(512),
  bio               TEXT,
  CONSTRAINT fk_speaker_sp
    FOREIGN KEY (participant_id) REFERENCES participants(id)
    ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Sponsor details (1:1 with a participant)
-- ----------------------------
CREATE TABLE sponsors (
  id                BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  participant_id    BIGINT UNSIGNED NOT NULL,
  tier              VARCHAR(80) NOT NULL,  -- normalize later if needed
  company_name      VARCHAR(255),
  logo_url          VARCHAR(512),
  invoice_contact   VARCHAR(255),
  benefits_notes    TEXT,
  created_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_sponsor_sp
    FOREIGN KEY (participant_id) REFERENCES participants(id)
    ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Exhibitor details (1:1 with a participant)
-- ----------------------------
CREATE TABLE exhibitors (
  id                BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  participant_id    BIGINT UNSIGNED NOT NULL,
  company_name      VARCHAR(255) NOT NULL,
  power_required    BOOLEAN NOT NULL DEFAULT false,
  internet_required BOOLEAN NOT NULL DEFAULT false,
  notes_internal    TEXT,
  created_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_exhibitor_sp
    FOREIGN KEY (participant_id) REFERENCES participants(id)
    ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

/* ===========================
   Activities (social/extras)
   =========================== */
CREATE TABLE activities (
  id                 BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  conference_id      BIGINT UNSIGNED NOT NULL,
  name               VARCHAR(255) NOT NULL,
  description        TEXT,
  start_at           DATETIME NOT NULL,
  end_at             DATETIME NOT NULL,
  venue_id           BIGINT UNSIGNED NULL,         -- optional: offsite location
  provider_url       VARCHAR(512),                 -- e.g., cruise company website
  capacity           INT NULL,                     -- NULL = unlimited
  created_at         DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at         DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_activities_conf FOREIGN KEY (conference_id)
    REFERENCES conferences(id) ON DELETE CASCADE,
  CONSTRAINT fk_activities_venue FOREIGN KEY (venue_id)
    REFERENCES venues(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_activities_conf_time ON activities(conference_id, start_at, end_at);

/* Bookings for activities (any participant can join) */
CREATE TABLE IF NOT EXISTS activity_bookings (
  id                        BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  activity_id               BIGINT UNSIGNED NOT NULL,
  participant_id            BIGINT UNSIGNED NOT NULL,
  status                    ENUM('reserved','confirmed','cancelled')
                               NOT NULL DEFAULT 'reserved',
  created_at                DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at                DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  UNIQUE KEY uq_activity_booking (activity_id, participant_id),
  CONSTRAINT fk_ab_activity FOREIGN KEY (activity_id)
    REFERENCES activities(id) ON DELETE CASCADE,
  CONSTRAINT fk_ab_sp FOREIGN KEY (participant_id)
    REFERENCES participants(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_ab_activity_status ON activity_bookings(activity_id, status);

/* Masterclasses (drop the single instructor FK) */
CREATE TABLE masterclasses (
  id            BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  conference_id BIGINT UNSIGNED NOT NULL,
  name          VARCHAR(255) NOT NULL,
  description   TEXT,
  start_at      DATETIME NOT NULL,
  end_at        DATETIME NOT NULL,
  venue_id      BIGINT UNSIGNED NULL,
  capacity      INT NULL,
  created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_mc_conf  FOREIGN KEY (conference_id)
    REFERENCES conferences(id) ON DELETE CASCADE,
  CONSTRAINT fk_mc_venue FOREIGN KEY (venue_id)
    REFERENCES venues(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_mc_conf_time ON masterclasses(conference_id, start_at, end_at);

/* NEW: Many-to-many instructors for a masterclass */
CREATE TABLE masterclass_instructors (
  masterclass_id BIGINT UNSIGNED NOT NULL,
  participant_id BIGINT UNSIGNED NOT NULL,
  is_lead        TINYINT(1) NOT NULL DEFAULT 0,
  created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (masterclass_id, participant_id),
  CONSTRAINT fk_mci_masterclass FOREIGN KEY (masterclass_id)
    REFERENCES masterclasses(id) ON DELETE CASCADE,
  CONSTRAINT fk_mci_participant FOREIGN KEY (participant_id)
    REFERENCES participants(id) ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

/* Bookings (fixed column name usage) */
CREATE TABLE masterclass_bookings (
  id              BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  masterclass_id  BIGINT UNSIGNED NOT NULL,
  participant_id  BIGINT UNSIGNED NOT NULL,
  status          ENUM('reserved','confirmed','cancelled') NOT NULL DEFAULT 'reserved',
  created_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  UNIQUE KEY uq_mc_booking (masterclass_id, participant_id),
  CONSTRAINT fk_mcb_masterclass FOREIGN KEY (masterclass_id)
    REFERENCES masterclasses(id) ON DELETE CASCADE,
  CONSTRAINT fk_mcb_participant FOREIGN KEY (participant_id)
    REFERENCES participants(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_mcb_mc_status ON masterclass_bookings(masterclass_id, status);
