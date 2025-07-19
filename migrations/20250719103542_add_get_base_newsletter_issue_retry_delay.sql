CREATE OR REPLACE FUNCTION get_base_newsletter_issue_retry_delay()
  RETURNS INTERVAL AS
  $$SELECT INTERVAL '1 minute' $$ LANGUAGE sql IMMUTABLE;