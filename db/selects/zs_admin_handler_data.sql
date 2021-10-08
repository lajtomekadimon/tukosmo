
CREATE OR REPLACE FUNCTION s_admin_handler_data(
    req "AdminRequest"
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
    WHEN userd IS NULL THEN NULL::"AdminDataDB"  -- You should be logged in
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
