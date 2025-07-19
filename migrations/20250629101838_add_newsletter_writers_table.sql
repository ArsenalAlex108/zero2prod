-- Add migration script here
CREATE TABLE newsletter_writers(
    user_id uuid PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    salted_password TEXT NOT NULL
)