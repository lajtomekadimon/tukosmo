
CREATE TYPE "NewUserPostARequest" AS (
    req "AdminRequest",
    name TEXT,
    email TEXT,
    password TEXT,
    repeat_password TEXT
);


CREATE OR REPLACE FUNCTION awa_new_user_post(
    r "NewUserPostARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    post_id BIGINT;

BEGIN

    -- Check request and select common data
    d := s_admin_handler_data(r.req);

    -- Check user name
    IF NOT e_is_user_name(r.name) THEN
        PERFORM err_wrong_user_name();
    END IF;

    -- Check email
    IF NOT e_is_email(r.email) THEN
        PERFORM err_wrong_email();
    END IF;

    -- Check email is unique
    IF c_user_by_email(r.email) THEN
        PERFORM err_email_already_exists();
    END IF;

    -- Check password
    IF NOT e_is_secure_password(r.password) THEN
        PERFORM err_wrong_password();
    END IF;

    -- Check password and repeat password are equal
    IF r.password <> r.repeat_password THEN
        PERFORM err_passwords_do_not_match();
    END IF;

    PERFORM i_user(
        r.email,
        CRYPT(r.password, GEN_SALT('bf')),
        r.name
    );

END;

$$;
