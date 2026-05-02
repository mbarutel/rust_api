CREATE TABLE IF NOT EXISTS clients (
  id               BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
  organization_id  BIGINT UNSIGNED,
  first_name       VARCHAR(120) NOT NULL,
  last_name        VARCHAR(120) NOT NULL,
  email            VARCHAR(255) NOT NULL,
  phone            VARCHAR(64),
  job_title        VARCHAR(160),
  created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  UNIQUE KEY uq_clients_email (email),
  CONSTRAINT fk_clients_org
    FOREIGN KEY (organization_id) REFERENCES organizations(id)
    ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE INDEX idx_clients_org ON clients(organization_id);
