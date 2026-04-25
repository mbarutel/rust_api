CREATE TABLE IF NOT EXISTS speakers (
  id                BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  participant_id    BIGINT UNSIGNED NOT NULL,
  talk_title        VARCHAR(255) NOT NULL,
  talk_abstract     TEXT,
  duration_minutes  INT,
  av_requirements   TEXT,
  headshot          VARCHAR(512),
  bio               TEXT,
  created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_speaker_participant
    FOREIGN KEY (participant_id) REFERENCES participants(id)
    ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
