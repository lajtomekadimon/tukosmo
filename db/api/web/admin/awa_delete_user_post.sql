
CREATE TYPE "DeleteUserPostARequest" AS (
    req "AdminRequest",
    csrf_token UUID,
    id BIGINT
);


CREATE OR REPLACE FUNCTION awa_delete_user_post(
    r "DeleteUserPostARequest"
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    d "AdminDataDB";

    language_of_user BIGINT;

BEGIN

    -- Check request
    d := s_admin_handler_data(r.req);

    language_of_user := (d.lang).id;

    -- Check CSRF token
    IF NOT c_csrf_token_by_token_session(
        r.csrf_token,
        (r.req).session
    ) THEN
        PERFORM err_wrong_csrf_token();
    END IF;

    -- Check user ID is correct
    IF s_user_by_id_lang(
        r.id,
        language_of_user
    ) IS NULL THEN
        PERFORM err_wrong_user_id();
    END IF;

    -- Check user is not deleting itself
    IF r.id = (d.userd).id THEN
        PERFORM err_user_cant_delete_itself();
    END IF;

    PERFORM d_user(r.id);

END;

$$;
