
CREATE OR REPLACE FUNCTION s_sessions_by_user(
    user_id BIGINT
)

RETURNS "SessionDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ARRAY(
    SELECT (
        -- user_agent
        ts_user_agent,

        -- date
        ts_date::TEXT
    )::"SessionDB"
    FROM t_sessions

    WHERE ts_user = user_id

    ORDER BY ts_date DESC
)

$$;
