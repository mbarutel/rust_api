CREATE TABLE IF NOT EXISTS registration (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  conference_id    BIGINT UNSIGNED NOT NULL,
  status           ENUM('submitted', 'accepted','waitlisted','rejected','withdrawn')
                     NOT NULL DEFAULT 'submitted',
  cost             DECIMAL(10,2) NOT NULL DEFAULT 0.00,
  discount_code    VARCHAR(64),
  discount_amount  DECIMAL(10,2) NOT NULL DEFAULT 0.00,
  amount_paid      DECIMAL(10,2) NOT NULL DEFAULT 0.00,
  created_by_id    BIGINT UNSIGNED,
  notes_internal   TEXT,
  created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_registration
_conf
    FOREIGN KEY (conference_id) REFERENCES conferences(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_registration
_created_by
    FOREIGN KEY (created_by_id) REFERENCES clients(id)
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_registration_conf ON registration(conference_id);
CREATE INDEX idx_registration_status ON registration(status);
