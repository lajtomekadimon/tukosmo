
CREATE OR REPLACE FUNCTION ss_stats_visits_yesterday()

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT COALESCE(
    SUM(tshv_visits),
    0
)
FROM tsh_visits
WHERE (NOW() - INTERVAL '2 days') <= tshv_date
    AND tshv_date <= (NOW() - INTERVAL '1 day')
    AND tshv_lang = ''
    AND tshv_route = ''

$$;
