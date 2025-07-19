ALTER TABLE issue_delivery_queue
    ADD COLUMN IF NOT EXISTS n_retries integer DEFAULT 0,
    ADD COLUMN IF NOT EXISTS execute_after INTERVAL DEFAULT now() - now();