
CREATE OR REPLACE FUNCTION s_csrf_token_by_session(
    session_id UUID
)

RETURNS UUID

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ts_csrf_token
FROM t_sessions
WHERE ts_id = session_id
LIMIT 1

$$;
