
CREATE TYPE "ApiDomain" AS (
    req "AdminRequest",
    csrf_token UUID,
    domain TEXT,
    email TEXT
);


CREATE OR REPLACE FUNCTION aha_p_domain(
    r "ApiDomain"
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

    -- Check domain
    IF NOT e_is_domain(r.domain) THEN
        PERFORM err_wrong_domain();
    END IF;

    -- Check email
    IF NOT e_is_email(r.email) THEN
        PERFORM err_wrong_email();
    END IF;

END;

$$;
