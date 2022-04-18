
CREATE OR REPLACE FUNCTION s_stats_server_in_hours(
    from_date TIMESTAMPTZ,
    to_date TIMESTAMPTZ
)

RETURNS "StatsServerDB"

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

    -- uploaded (NOTE: in MiB)
    STRING_AGG(
        COALESCE(
            (tshs_uploaded::DOUBLE PRECISION / 1048576)::TEXT,
            'NaN'
        ),
        ','
    ),

    -- downloaded (NOTE: in MiB)
    STRING_AGG(
        COALESCE(
            (tshs_downloaded::DOUBLE PRECISION / 1048576)::TEXT,
            'NaN'
        ),
        ','
    ),

    -- disk_used
    STRING_AGG(
        COALESCE(
            tshs_disk_used::TEXT,
            'NaN'
        ),
        ','
    ),

    -- disk_free
    STRING_AGG(
        COALESCE(
            tshs_disk_free::TEXT,
            'NaN'
        ),
        ','
    ),

    -- cpu
    STRING_AGG(
        COALESCE(
            tshs_cpu::TEXT,
            'NaN'
        ),
        ','
    ),

    -- memory
    STRING_AGG(
        COALESCE(
            tshs_memory::TEXT,
            'NaN'
        ),
        ','
    )
)::"StatsServerDB"

FROM (
    SELECT GENERATE_SERIES(
        MIN(from_date),
        MAX(to_date),
        '1 hour'
    )::TIMESTAMPTZ AS gdate
) x

LEFT JOIN tsh_server
ON tshs_date = x.gdate

$$;
