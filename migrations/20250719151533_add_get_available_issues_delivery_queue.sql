CREATE OR REPLACE FUNCTION get_available_issue_delivery_queue(timestamptz)
RETURNS setof issue_delivery_queue AS
'
SELECT newsletter_issue_id, subscriber_email, n_retries, execute_after, enabled
FROM
(
    SELECT
        issue_delivery_queue.newsletter_issue_id,
        issue_delivery_queue.subscriber_email,
        issue_delivery_queue.n_retries,
        issue_delivery_queue.execute_after,
        issue_delivery_queue.enabled,
        newsletter_issues.published_at
    FROM newsletter_issues
    INNER JOIN issue_delivery_queue
    ON newsletter_issues.newsletter_issue_id 
        = issue_delivery_queue.newsletter_issue_id
)
WHERE enabled = true
AND execute_after >= $1 - published_at;
' LANGUAGE SQL;