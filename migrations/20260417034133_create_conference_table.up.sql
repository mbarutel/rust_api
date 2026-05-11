CREATE TABLE IF NOT EXISTS conferences (
  id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  code VARCHAR(64) NOT NULL UNIQUE,
  name VARCHAR(255) NOT NULL,
  poster_url VARCHAR(512),
  description TEXT,
  start_date DATETIME,
  end_date DATETIME,
  venue_id BIGINT UNSIGNED,
  group_discount_id BIGINT UNSIGNED,
  published TINYINT(1) NOT NULL DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_conferences_venue FOREIGN KEY (venue_id) REFERENCES venues (id) ON DELETE SET NULL,
  CONSTRAINT fk_conferences_group_discount FOREIGN KEY (group_discount_id) REFERENCES group_discounts (id) ON DELETE SET NULL,
  INDEX idx_conferences_dates (start_date, end_date)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
