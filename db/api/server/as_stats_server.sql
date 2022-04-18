
CREATE TYPE "SaiStatsServer" AS (
    uploaded BIGINT,
    downloaded BIGINT,
    disk_used BIGINT,
    disk_free BIGINT,
    cpu DOUBLE PRECISION,
    memory DOUBLE PRECISION
);

CREATE OR REPLACE FUNCTION as_stats_server(
    stats_data "SaiStatsServer"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    date_value TIMESTAMPTZ;

BEGIN

    date_value := DATE_TRUNC('minute', NOW(), 'UTC');

    PERFORM i_stats_server(
        date_value,
        stats_data.uploaded,
        stats_data.downloaded,
        stats_data.disk_used,
        stats_data.disk_free,
        stats_data.cpu,
        stats_data.memory
    );

END;

$$;
