CREATE TABLE IF NOT EXISTS sponsors (
  id                BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  participant_id    BIGINT UNSIGNED NOT NULL,
  tier              VARCHAR(80) NOT NULL,
  company_name      VARCHAR(255),
  logo_url          VARCHAR(512),
  invoice_contact   VARCHAR(255),
  benefits_notes    TEXT,
  created_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_sponsor_participant
    FOREIGN KEY (participant_id) REFERENCES participants(id)
    ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
