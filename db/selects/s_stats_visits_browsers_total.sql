
CREATE OR REPLACE FUNCTION s_stats_visits_browsers_total()

RETURNS "StatsVisitsBrowsersDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    -- browsers
    STRING_AGG(
        temp.tshvb_browser,
        ','
    ),

    -- visitors
    STRING_AGG(
        COALESCE(
            sc_stats_visitors_total_by_browser(temp.tshvb_browser)::TEXT,
            '0'
        ),
        ','
    )
)::"StatsVisitsBrowsersDB"
FROM (
    SELECT DISTINCT tshvb_browser
    FROM tsh_visits_browsers
) temp

$$;
