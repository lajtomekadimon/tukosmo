
CREATE TYPE "ApiSessions" AS (
    req "AdminRequest",
    csrf_token UUID,
    user_agent TEXT,
    date TEXT
);


CREATE OR REPLACE FUNCTION aha_p_sessions(
    r "ApiSessions"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

BEGIN

    -- Check request
    d := s_admin_handler_data(r.req);

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;

    PERFORM d_session_by_user_agent_date(
        (d.userd).id,
        r.user_agent,
        r.date
    );

END;

$$;
