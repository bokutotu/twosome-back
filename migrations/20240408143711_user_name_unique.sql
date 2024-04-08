-- Add migration script here
ALTER TABLE users ADD COLUMN user_id TEXT UNIQUE NOT NULL DEFAULT gen_random_uuid();
