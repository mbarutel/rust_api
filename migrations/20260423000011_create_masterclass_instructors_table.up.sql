CREATE TABLE IF NOT EXISTS masterclass_instructors (
  masterclass_id BIGINT UNSIGNED NOT NULL,
  participant_id BIGINT UNSIGNED NOT NULL,
  is_lead        TINYINT(1) NOT NULL DEFAULT 0,
  created_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (masterclass_id, participant_id),
  CONSTRAINT fk_mci_masterclass
    FOREIGN KEY (masterclass_id) REFERENCES masterclasses(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_mci_participant
    FOREIGN KEY (participant_id) REFERENCES participants(id)
    ON DELETE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
