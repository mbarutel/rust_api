CREATE TABLE IF NOT EXISTS exhibitors (
  id                BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  participant_id    BIGINT UNSIGNED NOT NULL,
  company_name      VARCHAR(255) NOT NULL,
  power_required    BOOLEAN NOT NULL DEFAULT false,
  internet_required BOOLEAN NOT NULL DEFAULT false,
  notes_internal    TEXT,
  created_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_exhibitor_participant
    FOREIGN KEY (participant_id) REFERENCES participants(id)
    ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
