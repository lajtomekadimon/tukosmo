
CREATE OR REPLACE FUNCTION s_sum_tshs_uploaded_by_day(
    date_value TIMESTAMPTZ
)

RETURNS BIGINT

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT SUM(tshs_uploaded)
FROM tsh_server
WHERE tshs_date >= date_value
    AND tshs_date < (date_value + INTERVAL '1 day')

$$;
