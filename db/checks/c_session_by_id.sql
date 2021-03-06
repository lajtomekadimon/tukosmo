
CREATE OR REPLACE FUNCTION c_session_by_id(
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
    WHERE ts_id = session_id
    LIMIT 1
)

$$;
