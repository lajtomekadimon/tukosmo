
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
        -- ip
        ts_ip,

        -- browser
        ts_browser,

        -- platform
        ts_platform,

        -- date
        ts_date::TEXT
    )::"SessionDB"
    FROM t_sessions

    WHERE ts_user = user_id

    ORDER BY ts_date DESC
)

$$;
