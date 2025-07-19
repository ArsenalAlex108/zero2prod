-- Source: https://stackoverflow.com/a/26063344

CREATE OR REPLACE FUNCTION idempotency_age()
  RETURNS INTERVAL AS
  $$SELECT INTERVAL '1 hour' $$ LANGUAGE sql IMMUTABLE;

CREATE FUNCTION expire_idempotency() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  DELETE FROM idempotency WHERE created_at < NOW() - idempotency_age();
  RETURN NEW;
END;
$$;

CREATE TRIGGER expire_idempotency_trigger
    AFTER INSERT ON idempotency
    EXECUTE PROCEDURE expire_idempotency();