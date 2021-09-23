
CREATE OR REPLACE FUNCTION awa_admin_handler(

    session_id UUID,

    lang_code TEXT

)

RETURNS "AdminDataDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

WITH variables (userd, lang, languages) AS (
    VALUES (
        -- userd
        s_user_by_session_lang(
            session_id,
            s_language_id_by_code(lang_code)
        ),

        -- lang
        s_current_language_by_code(lang_code),

        -- languages
        s_languages(
            s_language_id_by_code(lang_code)
        )
    )
)

SELECT CASE
    WHEN userd IS NULL THEN NULL::"AdminDataDB"
    WHEN lang IS NULL THEN NULL::"AdminDataDB"
    WHEN CARDINALITY(languages) = 0 THEN NULL::"AdminDataDB"

    ELSE (
        userd,
        lang,
        languages
    )::"AdminDataDB"
END
FROM variables

$$;
