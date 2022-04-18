
CREATE OR REPLACE FUNCTION s_tshv_platform_id_by_visit_platform(
    visit_id BIGINT,
    platform_value TEXT
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT tshvp_id
FROM tsh_visits_platforms
WHERE tshvp_visit = visit_id
    AND tshvp_platform = platform_value
LIMIT 1

$$;
