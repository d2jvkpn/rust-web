-- set enum types
CREATE TYPE user_status AS ENUM('ok', 'blocked', 'deleted');
CREATE TYPE user_role AS ENUM('admin', 'leader', 'member');

-- setup trigger
CREATE FUNCTION updated_at() RETURNS trigger AS $$
BEGIN
  NEW.updated_at := now();
  RETURN NEW;
END;
$$LANGUAGE plpgsql;
-- drop function updated_at cascade;

CREATE TABLE users (
  id       serial       PRIMARY KEY,
  status   user_status  DEFAULT 'ok',
  role     user_role    NOT NULL,
  phone    varchar(20)  DEFAULT NULL UNIQUE,
  email    varchar(128) DEFAULT NULL UNIQUE,
  name     varchar(32)  NOT NULL,
  birthday char(10)     NOT NULL DEFAULT '',

  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);

-- ALTER TABLE users ADD CONSTRAINT name UNIQUE(name);
-- \d users

CREATE TRIGGER updated_at BEFORE INSERT OR UPDATE ON users
  FOR EACH ROW EXECUTE PROCEDURE updated_at();
