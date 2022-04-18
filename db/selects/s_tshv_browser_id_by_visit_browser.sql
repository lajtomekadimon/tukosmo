
CREATE OR REPLACE FUNCTION s_tshv_browser_id_by_visit_browser(
    visit_id BIGINT,
    browser_value TEXT
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tshvb_id
FROM tsh_visits_browsers
WHERE tshvb_visit = visit_id
    AND tshvb_browser = browser_value
LIMIT 1

$$;
