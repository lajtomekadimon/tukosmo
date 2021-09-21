
CREATE OR REPLACE FUNCTION aww_website_handler(

    session_id UUID,

    lang_code TEXT

)

RETURNS "WebsiteDataDB"

LANGUAGE SQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

WITH variables (userd, lang, languages) AS ( VALUES (
    -- userd
    CASE
        WHEN session_id IS NULL
        THEN NULL::"UserDB"
        ELSE (
            SELECT (id, email, name)::"UserDB"
            FROM s_user_by_session_lang(
                session_id,
                s_language_id_by_code(lang_code)
            )
        )::"UserDB"
    END,

    -- lang
    (
        SELECT (id, code, name, date, has_all_names)::"LanguageDB"
        FROM s_current_language_by_code(lang_code)
    )::"LanguageDB",

    -- languages
    (
        ARRAY(
            SELECT (id, code, name, date, has_all_names)::"LanguageDB"
            FROM s_languages(
                s_language_id_by_code(lang_code)
            )
        )
    )::"LanguageDB"[]
))

SELECT CASE
    WHEN lang IS NULL THEN NULL
    WHEN CARDINALITY(languages) = 0 THEN NULL
    ELSE (
        userd,
        lang,
        languages
    )::"WebsiteDataDB"
END
FROM variables

$$;
