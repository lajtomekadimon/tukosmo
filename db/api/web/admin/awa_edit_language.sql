
CREATE OR REPLACE FUNCTION awa_edit_language(
    language_of_user INT8,
    language_id INT8
)

RETURNS "NameDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ARRAY(
    SELECT (
        -- name
        name,

        -- lang
        (
            -- id
            lang_id,
            -- code
            lang_code,
            -- name
            lang_name,
            -- original_name
            lang_original_name,
            -- date
            lang_date,
            -- has_all_names
            lang_has_all_names
        )::"LanguageDB"
    )::"NameDB"

    FROM s_language_names(
        language_of_user,
        language_id
    )
)

$$;
