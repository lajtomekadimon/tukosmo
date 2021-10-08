
CREATE OR REPLACE FUNCTION s_admin_handler_data(
    req "AdminRequest"
)

RETURNS "AdminDataDB"

LANGUAGE PLPGSQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    userd "UserDB";

    lang "LanguageDB";

    languages "LanguageDB"[];

BEGIN

    userd := s_user_by_session_lang(
        (req).session,
        s_language_id_by_code((req).lang_code)
    );

    lang := s_current_language_by_code((req).lang_code);

    languages := s_languages(
        s_language_id_by_code((req).lang_code)
    );

    IF userd IS NULL THEN

        PERFORM err_user_not_logged_in();

    ELSIF lang IS NULL THEN

        RAISE EXCEPTION '';

    END IF;

    RETURN (
        userd,
        lang,
        languages
    )::"AdminDataDB";

END;

$$;
