
CREATE OR REPLACE FUNCTION ss_stats_visits_total()

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
WHERE tshv_lang = '' AND tshv_route = ''

$$;
