ALTER TABLE newsletter_issues
    DROP COLUMN published_at,
    ADD COLUMN published_at timestamptz NOT NULL DEFAULT now();