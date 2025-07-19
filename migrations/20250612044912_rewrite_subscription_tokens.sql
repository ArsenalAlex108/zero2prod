-- Add migration script here
DROP TABLE subscription_tokens;
CREATE TABLE IF NOT EXISTS subscription_tokens(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    subscriber_id uuid NOT NULL,
    FOREIGN KEY (subscriber_id) REFERENCES subscriptions (id)
);