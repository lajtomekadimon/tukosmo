
CREATE OR REPLACE FUNCTION s_website_handler_data(
    req "WebsiteRequest"
)

RETURNS "WebsiteDataDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

WITH variables (userd, lang, languages) AS (
    VALUES (
        -- userd
        s_user_by_session_lang(
            (req).session,
            s_language_id_by_code((req).lang_code)
        ),

        -- lang
        s_current_language_by_code((req).lang_code),

        -- languages
        s_languages(
            s_language_id_by_code((req).lang_code)
        )
    )
)

SELECT CASE
    WHEN lang IS NULL THEN NULL::"WebsiteDataDB"
    WHEN CARDINALITY(languages) = 0 THEN NULL::"WebsiteDataDB"

    ELSE (
        userd,
        lang,
        languages
    )::"WebsiteDataDB"
END
FROM variables

$$;

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

    userd "UserDB";

    lang "LanguageDB";

    languages "LanguageDB"[];

BEGIN

    userd := s_user_by_session_lang(
        (req).session,
        s_language_id_by_code((req).lang_code)
    );

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
