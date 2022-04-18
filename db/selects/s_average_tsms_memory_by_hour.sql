
CREATE OR REPLACE FUNCTION s_average_tsms_memory_by_hour(
    date_value TIMESTAMPTZ
)

RETURNS DOUBLE PRECISION

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT AVG(tsms_memory)
FROM tsm_server
WHERE tsms_date >= date_value
    AND tsms_date < (date_value + INTERVAL '1 hour')

$$;
