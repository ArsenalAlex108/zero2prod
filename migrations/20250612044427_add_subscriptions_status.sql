-- Add migration script here
ALTER TABLE subscriptions ADD COLUMN status varchar(255) NOT NULL DEFAULT 'pending_confirmation';
