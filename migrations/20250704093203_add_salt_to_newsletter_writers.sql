-- Add migration script here
ALTER TABLE newsletter_writers ADD COLUMN salt TEXT NOT NULL;