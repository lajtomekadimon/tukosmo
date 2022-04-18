
CREATE OR REPLACE FUNCTION s_average_tshs_cpu_by_day(
    date_value TIMESTAMPTZ
)

RETURNS DOUBLE PRECISION

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT AVG(tshs_cpu)
FROM tsh_server
WHERE tshs_date >= date_value
    AND tshs_date < (date_value + INTERVAL '1 day')

$$;
