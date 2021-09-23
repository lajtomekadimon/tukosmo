
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

WITH variables (userd, lang, languages) AS ( VALUES (
    -- userd
    (
        SELECT (id, email, name)::"UserDB"
        FROM s_user_by_session_lang(
            session_id,
            s_language_id_by_code(lang_code)
        )
    )::"UserDB",

    -- lang
    (
        SELECT (
            id,
            code,
            name,
            original_name,
            date,
            has_all_names
        )::"LanguageDB"
        FROM s_current_language_by_code(lang_code)
    )::"LanguageDB",

    -- languages
    (
        ARRAY(
            SELECT (
                id,
                code,
                name,
                original_name,
                date,
                has_all_names
            )::"LanguageDB"
            FROM s_languages(
                s_language_id_by_code(lang_code)
            )
        )
    )::"LanguageDB"[]
))

SELECT CASE
    WHEN userd IS NULL THEN NULL
    WHEN lang IS NULL THEN NULL
    WHEN CARDINALITY(languages) = 0 THEN NULL
    ELSE (
        userd,
        lang,
        languages
    )::"AdminDataDB"
END
FROM variables

$$;
