ALTER TABLE issue_delivery_queue
    ALTER COLUMN execute_after SET DEFAULT get_base_newsletter_issue_retry_delay()
