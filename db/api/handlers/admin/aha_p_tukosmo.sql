
CREATE TYPE "ApiTukosmo" AS (
    req "AdminRequest",
    csrf_token UUID
);


CREATE OR REPLACE FUNCTION aha_p_tukosmo(
    r "ApiTukosmo"
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

END;

$$;
