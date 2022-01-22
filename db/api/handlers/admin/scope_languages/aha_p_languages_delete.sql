
CREATE TYPE "ApiLanguagesDelete" AS (
    req "AdminRequest",
    csrf_token UUID,
    default_lang TEXT,
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

    lang_data "LanguageDB";

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

    lang_data := s_language_by_id_lang(
        r.id,
        language_of_user
    );

    -- Check language ID is correct
    IF lang_data IS NULL THEN
        PERFORM err_wrong_lang_id();
    END IF;

    -- Check the language is not default language
    IF lang_data.code = r.default_lang THEN
        PERFORM err_default_language_cant_be_deleted();
    END IF;

    PERFORM d_language(r.id);

END;

$$;
