
CREATE OR REPLACE FUNCTION i_stats_server(
    date_value TIMESTAMPTZ,
    uploaded_value BIGINT,
    downloaded_value BIGINT,
    disk_used BIGINT,
    disk_free BIGINT,
    cpu_value DOUBLE PRECISION,
    memory_value DOUBLE PRECISION
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    INSERT INTO tsm_server (
        tsms_date,
        tsms_uploaded,
        tsms_downloaded,
        tsms_disk_used,
        tsms_disk_free,
        tsms_cpu,
        tsms_memory
    )
    VALUES (
        date_value,
        uploaded_value,
        downloaded_value,
        disk_used,
        disk_free,
        cpu_value,
        memory_value
    );

END;

$$;
