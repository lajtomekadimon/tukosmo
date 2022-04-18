
CREATE OR REPLACE FUNCTION s_stats_server_in_min(
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
            (tsms_uploaded::DOUBLE PRECISION / 1048576)::TEXT,
            '0' -- TODO: Distinguish between online without data and offline
        ),
        ','
    ),

    -- downloaded (NOTE: in MiB)
    STRING_AGG(
        COALESCE(
            (tsms_downloaded::DOUBLE PRECISION / 1048576)::TEXT,
            '0' -- TODO: Distinguish between online without data and offline
        ),
        ','
    ),

    -- disk_used
    STRING_AGG(
        COALESCE(
            tsms_disk_used::TEXT,
            /* TODO: Can't use LAG() (previous value) inside an aggregate
               TODO: Distinguish between online without data and offline
            (LAG(tsms_disk_used, 1) OVER (
                ORDER BY tsms_date
            ))::TEXT,
            */
            'NaN'
        ),
        ','
    ),

    -- disk_free
    STRING_AGG(
        COALESCE(
            tsms_disk_free::TEXT,
            /* TODO: Can't use LAG() (previous value) inside an aggregate
               TODO: Distinguish between online without data and offline
            (LAG(tsms_disk_free, 1) OVER (
                ORDER BY tsms_date
            ))::TEXT,
            */
            'NaN'
        ),
        ','
    ),

    -- cpu
    STRING_AGG(
        COALESCE(
            tsms_cpu::TEXT,
            '0' -- TODO: Distinguish between online without data and offline
        ),
        ','
    ),

    -- memory
    STRING_AGG(
        COALESCE(
            tsms_memory::TEXT,
            '0' -- TODO: Distinguish between online without data and offline
        ),
        ','
    )
)::"StatsServerDB"

FROM (
    SELECT GENERATE_SERIES(
        MIN(from_date),
        MAX(to_date),
        '1 minute'
    )::TIMESTAMPTZ AS gdate
) x

LEFT JOIN tsm_server
ON tsms_date = x.gdate

$$;
