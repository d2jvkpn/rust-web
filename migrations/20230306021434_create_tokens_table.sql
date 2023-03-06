-- Add migration script here
CREATE TYPE platform AS ENUM('web', 'android', 'ios', 'unknown');

CREATE TABLE tokens (
  token_id    uuid         DEFAULT gen_random_uuid() PRIMARY KEY,
  user_id     int          NOT NULL,
  iat         bigint       NOT NULL,
  exp         bigint       NOT NULL,
  ip          inet         DEFAULT NULL,
  platform    platform     DEFAULT 'unknown',
  device      varchar(32)  DEFAULT NULL,
  status      boolean      DEFAULT true,
  updated_at  timestamptz  DEFAULT '0001-01-01 00:00:00Z'
);

CREATE TRIGGER tokens_updated_at BEFORE UPDATE ON tokens
  FOR EACH ROW EXECUTE PROCEDURE updated_at();
-- 

CREATE INDEX user_tokens_key ON tokens (user_id, exp DESC, status);
-- DROP INDEX user_tokens_key;

-- gen_random_uuid()
INSERT INTO tokens (user_id, iat, exp, ip, platform) VALUES
  (42, 1678005219, 1678007019, '127.0.0.1', 'web');

-- explain select * from tokens where user_id = 42 and exp > 1678007000 and status and platform = 'unknown';
