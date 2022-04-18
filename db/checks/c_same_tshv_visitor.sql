
CREATE OR REPLACE FUNCTION c_same_tshv_visitor(
    visit_id BIGINT,
    ip_value TEXT
)

RETURNS BOOL

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT EXISTS(
    SELECT 1
    FROM tsh_visits_visitors
    WHERE tshvv_visit = visit_id
        AND tshvv_ip = ip_value
    LIMIT 1
)

$$;
