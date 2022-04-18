
CREATE OR REPLACE FUNCTION s_stats_visits_platforms_total()

RETURNS "StatsVisitsPlatformsDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    -- platforms
    STRING_AGG(
        temp.tshvp_platform,
        ','
    ),

    -- visitors
    STRING_AGG(
        COALESCE(
            sc_stats_visitors_total_by_platform(temp.tshvp_platform)::TEXT,
            '0'
        ),
        ','
    )
)::"StatsVisitsPlatformsDB"
FROM (
    SELECT DISTINCT tshvp_platform
    FROM tsh_visits_platforms
) temp

$$;
