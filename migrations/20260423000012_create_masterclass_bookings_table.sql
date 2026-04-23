CREATE TABLE IF NOT EXISTS masterclass_bookings (
  id             BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  masterclass_id BIGINT UNSIGNED NOT NULL,
  participant_id BIGINT UNSIGNED NOT NULL,
  status         ENUM('reserved','confirmed','cancelled') NOT NULL DEFAULT 'reserved',
  created_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  UNIQUE KEY uq_mc_booking (masterclass_id, participant_id),
  CONSTRAINT fk_mcb_masterclass
    FOREIGN KEY (masterclass_id) REFERENCES masterclasses(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_mcb_participant
    FOREIGN KEY (participant_id) REFERENCES participants(id)
    ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_mcb_mc_status ON masterclass_bookings(masterclass_id, status);
