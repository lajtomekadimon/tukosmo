
CREATE OR REPLACE FUNCTION s_languages(
    language_of_user BIGINT
)

RETURNS "LanguageDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ARRAY(
    SELECT (
        -- id
        tl_id,

        -- code
        tl_code,

        -- name
        COALESCE(
            tln_name,
            '[untranslated: ' || tl_code || ']'
        ),

        -- original_name
        COALESCE(
            (
                SELECT b.tln_name
                FROM t_language_names b
                WHERE b.tln_lang = tl_id
                    AND b.tln_name_lang = tl_id
            ),
            '[untranslated: ' || tl_code || ']'
        ),

        -- date
        COALESCE(tln_date::TEXT, ''),

        -- has_all_names
        c_language_has_all_names(tl_id)
    )::"LanguageDB"

    FROM t_languages
    LEFT JOIN t_language_names
    ON tl_id = tln_lang
        AND tln_name_lang = language_of_user
    ORDER BY tln_name
)

$$;
