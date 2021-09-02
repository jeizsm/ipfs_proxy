-- Add migration script here
CREATE TABLE api_keys (id bigserial PRIMARY KEY, api_key uuid NOT NULL, user_id bigint NOT NULL, enabled bool NOT NULL DEFAULT true, FOREIGN KEY(user_id) REFERENCES users(id), CONSTRAINT uniq_api_key UNIQUE(api_key));
