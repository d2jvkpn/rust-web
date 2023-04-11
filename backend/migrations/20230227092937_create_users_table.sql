-- set enum types
CREATE TYPE user_status AS ENUM('ok', 'frozen', 'blocked', 'deleted');
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
  id          serial        PRIMARY KEY,
  status      user_status   NOT NULL,
  role        user_role     NOT NULL,
  phone       varchar(20)   DEFAULT NULL UNIQUE,
  email       varchar(128)  DEFAULT NULL UNIQUE,
  name        varchar(32)   NOT NULL,
  birthday    char(10)      DEFAULT NULL,
  password    varchar(64)   DEFAULT NULL,
  created_at  timestamptz   NOT NULL DEFAULT now(),
  updated_at  timestamptz   NOT NULL DEFAULT now()
);

-- ALTER TABLE users ADD CONSTRAINT name UNIQUE(name);
-- \d users

--  BEFORE INSERT OR UPDATE ON
CREATE TRIGGER users_updated_at BEFORE UPDATE ON users
  FOR EACH ROW EXECUTE PROCEDURE updated_at();

-- password: 12QWas!@, don't set id = 1 here as users_id_seq not called
INSERT INTO users (status, role, email, name, birthday, password) VALUES
  ('ok', 'admin', 'admin@noreply.local', 'admin', '2006-01-02',
    '$2b$12$9tK/XV7r4yXRQVm2jYshieKgr.CsFDVD7YxQRUt2FF5TBnIt7Phx.')
  ON CONFLICT DO NOTHING;
-- DO UPDATE
