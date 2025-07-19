-- Add migration script here
CREATE TABLE IF NOT EXISTS subscription_tokens(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (id) REFERENCES subscriptions (id),
    subscription_token uuid NOT NULL
)