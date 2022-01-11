
CREATE OR REPLACE FUNCTION s_website_handler_data(
    req "WebsiteRequest"
)

RETURNS "WebsiteDataDB"

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

    website_title_value TEXT;
    website_subtitle_value TEXT;
    copyright_owner_value TEXT;

BEGIN

    lang := s_current_language_by_code((req).lang_code);

    -- Check language code is correct
    IF lang IS NULL THEN
        PERFORM err_wrong_lang_code();
    END IF;

    language_of_user := lang.id;

    userd := s_user_by_session_lang(
        (req).session,
        language_of_user
    );

    languages := s_languages(language_of_user);

    website_title_value := s_website_title_by_lang(language_of_user);
    website_subtitle_value := s_website_subtitle_by_lang(language_of_user);
    copyright_owner_value := s_copyright_owner_by_lang(language_of_user);

    RETURN (
        userd,
        lang,
        languages,
        website_title_value,
        website_subtitle_value,
        copyright_owner_value
    )::"WebsiteDataDB";

END;

$$;
