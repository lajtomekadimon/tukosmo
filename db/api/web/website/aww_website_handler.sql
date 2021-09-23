
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

WITH variables (userd, lang, languages) AS (
    VALUES (
        -- userd
        CASE
            WHEN session_id IS NULL
            THEN NULL::"UserDB"
            ELSE s_user_by_session_lang(
                session_id,
                s_language_id_by_code(lang_code)
            )
        END,

        -- lang
        s_current_language_by_code(lang_code),

        -- languages
        s_languages(
            s_language_id_by_code(lang_code)
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
