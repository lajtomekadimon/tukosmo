
CREATE OR REPLACE FUNCTION s_user_names_by_id(
    user_id BIGINT,
    language_of_user BIGINT
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
        COALESCE(
            tun_name,
            ''
        ),

        -- lang
        (
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
    )::"NameDB"

    FROM t_user_names

    INNER JOIN t_languages
    ON tun_lang = tl_id

    LEFT JOIN t_language_names
    ON tl_id = tln_lang
        AND tln_name_lang = language_of_user

    ORDER BY tln_name
)

$$;
