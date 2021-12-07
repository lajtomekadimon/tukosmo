
CREATE TYPE "FaviconPostARequest" AS (
    req "AdminRequest"
    --csrf_token UUID
);


CREATE OR REPLACE FUNCTION awa_favicon_post(
    r "FaviconPostARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    file_id BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Check CSRF token
    /*
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;
    */

END;

$$;
