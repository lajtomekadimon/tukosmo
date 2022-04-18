
CREATE OR REPLACE FUNCTION sc_stats_visitors_today()

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT COUNT(*)
FROM (
    SELECT DISTINCT tshvv_ip
    FROM tsh_visits
    INNER JOIN tsh_visits_visitors
    ON tshv_id = tshvv_visit
    WHERE (NOW() - INTERVAL '1 day') <= tshv_date
        AND tshv_lang = ''
        AND tshv_route = ''
) temp

$$;
