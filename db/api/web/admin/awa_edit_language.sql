
CREATE OR REPLACE FUNCTION awa_edit_language(
    language_id BIGINT,
    language_of_user BIGINT
)

RETURNS "LanguageWithNamesDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    id,
    code,
    name,
    original_name,
    date,
    has_all_names,

    -- names
    s_language_names(
        language_of_user,
        language_id
    )
)::"LanguageWithNamesDB"

FROM s_language_by_id_lang(
    language_id,
    language_of_user
)

$$;
