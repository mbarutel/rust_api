CREATE TABLE IF NOT EXISTS submissions (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  conference_id    BIGINT UNSIGNED NOT NULL,
  status           ENUM('draft','submitted','under_review','accepted','waitlisted','rejected','withdrawn')
                     NOT NULL DEFAULT 'draft',
  created_by_id    BIGINT UNSIGNED,
  notes_internal   TEXT,
  created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_submissions_conf
    FOREIGN KEY (conference_id) REFERENCES conferences(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_submissions_created_by
    FOREIGN KEY (created_by_id) REFERENCES clients(id)
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_submissions_conf ON submissions(conference_id);
CREATE INDEX idx_submissions_status ON submissions(status);
