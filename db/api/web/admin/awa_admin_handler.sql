
CREATE OR REPLACE FUNCTION awa_admin_handler(

    session_id UUID,

    lang_code TEXT

)

RETURNS "DataDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

-- TODO: The only allowed NULL value is for data_db, the rest can't be NULL
SELECT (
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
)::"DataDB"

$$;
