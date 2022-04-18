
CREATE OR REPLACE FUNCTION s_stats_visits_referrals_total()

RETURNS "StatsVisitsReferralsDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    -- referrers
    STRING_AGG(
        temp.tshvr_referrer,
        ','
    ),

    -- visitors
    STRING_AGG(
        COALESCE(
            sc_stats_visitors_total_by_referrer(temp.tshvr_referrer)::TEXT,
            '0'
        ),
        ','
    ),

    -- visits
    STRING_AGG(
        COALESCE(
            temp.visits::TEXT,
            '0'
        ),
        ','
    )
)::"StatsVisitsReferralsDB"
FROM (
    SELECT
        tshvr_referrer,
        SUM(tshvr_visits) AS visits
    FROM tsh_visits
    INNER JOIN tsh_visits_referrals
    ON tshv_id = tshvr_visit
    WHERE tshv_lang = '' AND tshv_route = ''
    GROUP BY tshvr_referrer
    ORDER BY visits DESC
    LIMIT 8
) temp

$$;
