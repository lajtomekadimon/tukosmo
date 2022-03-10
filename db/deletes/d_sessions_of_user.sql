
CREATE OR REPLACE FUNCTION d_sessions_of_user(
    user_id BIGINT
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_sessions
    WHERE ts_user = user_id;

END;

$$;
