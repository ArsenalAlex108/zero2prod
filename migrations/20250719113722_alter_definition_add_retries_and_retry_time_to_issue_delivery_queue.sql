ALTER TABLE issue_delivery_queue
    ALTER COLUMN n_retries SET NOT NULL,
    ALTER COLUMN execute_after SET NOT NULL,
    ADD COLUMN IF NOT EXISTS enabled boolean NOT NULL DEFAULT true;
