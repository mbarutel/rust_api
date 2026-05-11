-- create_price_tier_table.up.sql
CREATE TABLE IF NOT EXISTS price_tiers (
  id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  conference_id BIGINT UNSIGNED NOT NULL,
  price DECIMAL(19, 4) NOT NULL,
  deadline DATE NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CONSTRAINT fk_price_tiers_conference FOREIGN KEY (conference_id) REFERENCES conferences (id) ON DELETE CASCADE,
  INDEX idx_price_tiers_conference_id (conference_id)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
