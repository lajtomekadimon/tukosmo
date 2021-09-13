
CREATE OR REPLACE FUNCTION i_session_by_email(

    user_email TEXT,

    user_agent_value TEXT

)

RETURNS UUID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    session_id UUID;

    user_id BIGINT;

BEGIN

    session_id := GEN_RANDOM_UUID();

    user_id := s_user_id_by_email(user_email);

    INSERT INTO t_sessions (
        ts_id,
        ts_user,
        ts_user_agent
        --ts_date
    ) VALUES (
        session_id,
        user_id,
        user_agent_value
        --NOW()
    );

    RETURN session_id;

END;

$$;
