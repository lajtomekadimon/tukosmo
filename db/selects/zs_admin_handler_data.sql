
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

    language_of_user BIGINT;

    userd "UserDB";

    lang "LanguageDB";

    languages "LanguageDB"[];

BEGIN

    language_of_user := s_language_id_by_code((req).lang_code);

    userd := s_user_by_session_lang(
        (req).session,
        language_of_user
    );

    IF userd IS NULL THEN
        PERFORM err_user_not_logged_in();
    END IF;

    lang := s_current_language_by_code((req).lang_code);

    IF lang IS NULL THEN
        PERFORM err_wrong_lang_code();
    END IF;

    languages := s_languages(language_of_user);

    RETURN (
        userd,
        lang,
        languages
    )::"AdminDataDB";

END;

$$;
