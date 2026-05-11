CREATE TABLE IF NOT EXISTS activities (
  id                 BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  conference_id      BIGINT UNSIGNED NOT NULL,
  name               VARCHAR(255) NOT NULL,
  description        TEXT,
  start_at           DATETIME NOT NULL,
  end_at             DATETIME NOT NULL,
  venue_id           BIGINT UNSIGNED,
  provider_url       VARCHAR(512),
  capacity           INT,
  created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_activities_conf
    FOREIGN KEY (conference_id) REFERENCES conferences(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_activities_venue
    FOREIGN KEY (venue_id) REFERENCES venues(id)
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_activities_conf_time ON activities(conference_id, start_at, end_at);
