
CREATE TYPE "AgiLogout" AS (
    req "AdminRequest"
);


CREATE OR REPLACE FUNCTION aha_g_logout(
    r "AgiLogout"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    -- Check request
    PERFORM s_admin_handler_data(r.req);

    PERFORM d_session((r.req).session);

END;

$$;
