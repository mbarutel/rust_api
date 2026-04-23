CREATE TABLE IF NOT EXISTS participants (
  id                   BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  submission_id        BIGINT UNSIGNED NOT NULL,
  client_id            BIGINT UNSIGNED NOT NULL,
  participant_role     ENUM('delegate','speaker','sponsor','exhibitor')
                         NOT NULL DEFAULT 'delegate',
  dietary_requirements TEXT,
  accessibility_needs  TEXT,
  UNIQUE KEY uq_submission_client (submission_id, client_id),
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
