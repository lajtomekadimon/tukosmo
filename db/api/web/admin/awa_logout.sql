
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

    IF s_admin_handler_data(r.req) IS NULL THEN

        RAISE EXCEPTION 'TODO: Wrong request or user not logged in.';

    END IF;

    PERFORM d_session((r.req).session);

END;

$$;
