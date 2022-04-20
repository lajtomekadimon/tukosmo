
CREATE OR REPLACE FUNCTION i_session(
    user_id BIGINT,
    ip_value TEXT,
    browser_value TEXT,
    platform_value TEXT
)

RETURNS UUID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    session_id UUID;

BEGIN

    session_id := GEN_RANDOM_UUID();

    INSERT INTO t_sessions (
        ts_id,
        ts_user,
        ts_ip,
        ts_browser,
        ts_platform
        --ts_date
    ) VALUES (
        session_id,
        user_id,
        ip_value,
        browser_value,
        platform_value
        --NOW()
    );

    RETURN session_id;

END;

$$;
