
CREATE TYPE "EditUserPostARequest" AS (
    req "AdminRequest",
    id BIGINT,
    email TEXT,
    name TEXT
);


CREATE OR REPLACE FUNCTION awa_edit_user_post(
    r "EditUserPostARequest"
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

    -- TODO: Check user ID is correct

    -- Check user name
    IF NOT e_is_user_name(r.name) THEN
        PERFORM err_wrong_user_name();
    END IF;

    -- Check email
    IF NOT e_is_email(r.email) THEN
        PERFORM err_wrong_email();
    END IF;

    -- Check email is unique
    IF c_user_by_email(r.email)
        AND r.id <> s_user_id_by_email(r.email)
    THEN
        PERFORM err_email_already_exists();
    END IF;

    PERFORM u_user_by_admin(
        r.id,
        r.email,
        r.name
    );

END;

$$;
