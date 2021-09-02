-- Add migration script here
CREATE TABLE logs (id bigserial PRIMARY KEY, api_key_id bigint NOT NULL, time timestamp NOT NULL DEFAULT NOW(), FOREIGN KEY(api_key_id) REFERENCES api_keys(id));
