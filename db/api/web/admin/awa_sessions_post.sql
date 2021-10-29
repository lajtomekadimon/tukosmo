
CREATE TYPE "SessionsPostARequest" AS (
    req "AdminRequest",
    user_agent TEXT,
    date TEXT
);


CREATE OR REPLACE FUNCTION awa_sessions_post(
    r "SessionsPostARequest"
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

    PERFORM d_session_by_user_agent_date(
        (d.userd).id,
        r.user_agent,
        r.date
    );

END;

$$;
