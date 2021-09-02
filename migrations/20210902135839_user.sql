-- Add migration script here
CREATE TABLE users (id bigserial PRIMARY KEY, username text NOT NULL, password text NOT NULL, CONSTRAINT uniq_username UNIQUE(username));
