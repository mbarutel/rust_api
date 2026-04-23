CREATE TABLE IF NOT EXISTS masterclasses (
  id            BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  conference_id BIGINT UNSIGNED NOT NULL,
  name          VARCHAR(255) NOT NULL,
  description   TEXT,
  start_at      DATETIME NOT NULL,
  end_at        DATETIME NOT NULL,
  venue_id      BIGINT UNSIGNED,
  capacity      INT,
  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_mc_conf
    FOREIGN KEY (conference_id) REFERENCES conferences(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_mc_venue
    FOREIGN KEY (venue_id) REFERENCES venues(id)
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_mc_conf_time ON masterclasses(conference_id, start_at, end_at);
