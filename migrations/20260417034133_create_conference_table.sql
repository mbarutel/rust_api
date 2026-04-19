CREATE TABLE IF NOT EXISTS conferences (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  code             VARCHAR(64) NOT NULL UNIQUE,
  name             VARCHAR(255) NOT NULL,
  poster_url       VARCHAR(512),
  description      TEXT,
  start_date       DATE,
  end_date         DATE,
  venue_id         BIGINT UNSIGNED,
  published        TINYINT(1) NOT NULL DEFAULT 0,
  created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_conferences_venue
    FOREIGN KEY (venue_id) REFERENCES venues(id)
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_conferences_dates ON conferences(start_date, end_date);
