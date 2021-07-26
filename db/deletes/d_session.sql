
CREATE OR REPLACE FUNCTION d_session(

    session_id UUID

)

RETURNS UUID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_sessions
    WHERE ts_id = session_id;

    RETURN session_id;

END;

$$;
