CREATE TABLE IF NOT EXISTS activity_bookings (
  id             BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  activity_id    BIGINT UNSIGNED NOT NULL,
  participant_id BIGINT UNSIGNED NOT NULL,
  status         ENUM('reserved','confirmed','cancelled') NOT NULL DEFAULT 'reserved',
  created_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  UNIQUE KEY uq_activity_booking (activity_id, participant_id),
  CONSTRAINT fk_ab_activity
    FOREIGN KEY (activity_id) REFERENCES activities(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_ab_participant
    FOREIGN KEY (participant_id) REFERENCES participants(id)
    ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_ab_activity_status ON activity_bookings(activity_id, status);
