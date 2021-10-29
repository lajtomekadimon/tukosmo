
CREATE OR REPLACE FUNCTION d_session_by_user_agent_date(
    user_id BIGINT,
    user_agent_value TEXT,
    date_value TEXT
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_sessions
    WHERE ts_user = user_id
        AND ts_user_agent = user_agent_value
        AND ts_date::TEXT = date_value;

END;

$$;
