
CREATE TYPE "ApiForgottenPassword" AS (
    req "WebsiteRequest",
    email TEXT
);

CREATE TYPE "ApoForgottenPassword" AS (
    data "WebsiteDataDB",
    code TEXT
);


CREATE OR REPLACE FUNCTION aha_p_forgotten_password(
    r "ApiForgottenPassword"
)

RETURNS "ApoForgottenPassword"

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    handler_data "WebsiteDataDB";

    user_id BIGINT;

    reset_password_code_id UUID;

BEGIN

    -- Check request and select common data
    handler_data := s_website_handler_data(r.req);

    user_id := s_user_id_by_email(r.email);

    -- Check email exists
    IF user_id IS NULL THEN
        PERFORM err_wrong_user_email();
    END IF;

    reset_password_code_id := i_reset_password_code_by_user_id(user_id);

    RETURN ROW(
        -- data
        handler_data,

        -- code
        reset_password_code_id::TEXT
    );

END;

$$;
