
CREATE OR REPLACE FUNCTION sc_stats_visitors_total_by_browser(
    browser_value TEXT
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT COUNT(*)
FROM (
    SELECT DISTINCT tshvv_ip
    FROM tsh_visits_visitors
    WHERE tshvv_browser = browser_value
) temp

$$;
