
CREATE OR REPLACE FUNCTION s_stats_visits_in_hours(
    from_date TIMESTAMPTZ,
    to_date TIMESTAMPTZ
)

RETURNS "StatsVisitsDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    -- date
    STRING_AGG(
        x.gdate::TEXT,
        ',' ORDER BY x.gdate
    ),

    -- visitors
    STRING_AGG(
        COALESCE(
            tshv_visitors::TEXT,
            '0'
        ),
        ','
    ),

    -- visits
    STRING_AGG(
        COALESCE(
            tshv_visits::TEXT,
            '0'
        ),
        ','
    )
)::"StatsVisitsDB"

FROM (
    SELECT GENERATE_SERIES(
        MIN(from_date),
        MAX(to_date),
        '1 hour'
    )::TIMESTAMPTZ AS gdate
) x

LEFT JOIN tsh_visits
ON tshv_date = x.gdate
    AND tshv_lang = ''
    AND tshv_route = ''

$$;
