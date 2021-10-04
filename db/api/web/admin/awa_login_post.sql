
CREATE TYPE "LoginPostARequest" AS (
    req "WebsiteRequest",
    email TEXT,
    password TEXT,
    user_agent TEXT
);

CREATE TYPE "LoginPostAResponse" AS (
    data "WebsiteDataDB",
    session UUID
);


CREATE OR REPLACE FUNCTION awa_login_post(
    r "LoginPostARequest"
)

RETURNS "LoginPostAResponse"

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    handler_data "WebsiteDataDB";

    real_user_password TEXT;

    session_id UUID;

BEGIN

    handler_data := s_website_handler_data(r.req);

    IF handler_data.lang IS NULL THEN

        RAISE EXCEPTION 'TODO: Bad request (probably lang_code).';

    END IF;

    real_user_password := s_user_password_by_email(r.email);

    -- The user exists
    IF real_user_password IS NOT NULL THEN

        -- Password is correct
        /* TODO: Currently, password check is done here, in the
         * database. It may be more secure to do this
         * in the web server, but the downside is that
         * instead of one query, we would need to do two
         * queries from there: one for the password check, and
         * another one for the new session.
         */
        IF real_user_password = CRYPT(r.password, real_user_password) THEN

            session_id := i_session_by_email(
                r.email,
                r.user_agent
            );

        ELSE

            RAISE EXCEPTION 'TODO: User exists but password is not valid.';

        END IF;

    ELSE

        RAISE EXCEPTION 'TODO: No user has that email.';

    END IF;

    RETURN ROW(
        -- data
        handler_data,

        -- session
        session_id
    );

END;

$$;
