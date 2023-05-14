-- Add migration script here
CREATE TABLE chats (
  request_id   uuid         NOT NULL,
  user_id      int          NOT NULL,
  query        text         NOT NULL,
  query_at     timestamptz  NOT NULL,
  response     text         DEFAULT NULL,
  response_at  timestamptz  DEFAULT NULL
);

CREATE INDEX chats_query ON chats (user_id, query_at DESC);
