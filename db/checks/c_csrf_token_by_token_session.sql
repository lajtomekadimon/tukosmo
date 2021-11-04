
CREATE OR REPLACE FUNCTION c_csrf_token_by_token_session(
    csrf_token_value UUID,
    session_id UUID
)

RETURNS BOOL

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT EXISTS(
    SELECT 1
    FROM t_sessions
    WHERE ts_csrf_token = csrf_token_value
        AND ts_id = session_id
    LIMIT 1
)

$$;
