
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

    IF userd IS NULL THEN
        PERFORM err_user_not_logged_in();
    END IF;

    lang := s_current_language_by_code((req).lang_code);

    IF lang IS NULL THEN
        PERFORM err_wrong_lang_code();
    END IF;

    languages := s_languages(
        s_language_id_by_code((req).lang_code)
    );

    RETURN (
        userd,
        lang,
        languages
    )::"AdminDataDB";

END;

$$;
