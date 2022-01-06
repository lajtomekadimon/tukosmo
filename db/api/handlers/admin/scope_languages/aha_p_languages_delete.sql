
CREATE TYPE "ApiLanguagesDelete" AS (
    req "AdminRequest",
    csrf_token UUID,
    id BIGINT
);


CREATE OR REPLACE FUNCTION aha_p_languages_delete(
    r "ApiLanguagesDelete"
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

    -- Check language ID is correct
    IF s_language_by_id_lang(
        r.id,
        language_of_user
    ) IS NULL THEN
        PERFORM err_wrong_lang_id();
    END IF;

    PERFORM d_language(r.id);

END;

$$;
