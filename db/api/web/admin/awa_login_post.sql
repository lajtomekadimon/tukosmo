
CREATE OR REPLACE FUNCTION awa_login_post(

    user_email TEXT,

    user_password TEXT,

    user_agent_value TEXT

)

RETURNS UUID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    real_user_password TEXT;

    session_id UUID;

BEGIN

    real_user_password := s_user_password_by_email(user_email);

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
        IF real_user_password = CRYPT(user_password, real_user_password) THEN

            session_id := i_session_by_email(
                user_email,
                user_agent_value
            );

        -- User exists but password is not correct
        ELSE

            RAISE EXCEPTION 'TODO';

        END IF;

    -- No user has that email
    ELSE

        RAISE EXCEPTION 'TODO';

    END IF;

    RETURN session_id;

END;

$$;
