CREATE TABLE IF NOT EXISTS venues (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  name             VARCHAR(255) NOT NULL,
  address_line1    VARCHAR(255),
  address_line2    VARCHAR(255),
  city             VARCHAR(120),
  state_region     VARCHAR(120),
  postal_code      VARCHAR(40),
  country          VARCHAR(120),
  notes            TEXT,
  created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
