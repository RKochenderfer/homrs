-- Your SQL goes here
CREATE UNIQUE INDEX IF NOT EXISTS index_token
ON sessions (token)