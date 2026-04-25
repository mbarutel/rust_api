CREATE TABLE IF NOT EXISTS participants (
  id                   BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  registration_id      BIGINT UNSIGNED NOT NULL,
  client_id            BIGINT UNSIGNED NOT NULL,
  participant_role     ENUM('delegate','speaker','sponsor','exhibitor')
                         NOT NULL DEFAULT 'delegate',
  dietary_requirements TEXT,
  accessibility_needs  TEXT,
  created_at           TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at           TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  UNIQUE KEY uq_registration_client (registration_id, client_id),
  CONSTRAINT fk_sp_registration
    FOREIGN KEY (registration_id) REFERENCES registration(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_sp_client
    FOREIGN KEY (client_id) REFERENCES clients(id)
    ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_sp_registration ON participants(registration_id);
CREATE INDEX idx_sp_client ON participants(client_id);
CREATE INDEX idx_sp_role ON participants(participant_role);
