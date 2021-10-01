
CREATE TYPE "LogoutARequest" AS (
    req "AdminRequest"
);


CREATE OR REPLACE FUNCTION awa_logout(
    r "LogoutARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    -- If the user is logged in...
    IF s_admin_handler_data(r.req) IS NOT NULL THEN

        PERFORM d_session((r.req).session);

    ELSE

        RAISE EXCEPTION 'TODO';

    END IF;

END;

$$;
