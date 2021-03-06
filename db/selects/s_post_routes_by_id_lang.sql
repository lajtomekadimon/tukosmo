
CREATE OR REPLACE FUNCTION s_post_routes_by_id_lang(
    post_id BIGINT,
    language_of_user BIGINT
)

RETURNS "RouteDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$


SELECT ARRAY(
    SELECT (
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
        )::"LanguageDB",

        '/' || tl_code || '/blog/' || tpt_permalink
    )::"RouteDB"
    FROM t_post_translations

    INNER JOIN t_languages
    ON tl_id = tpt_lang

    LEFT JOIN t_language_names
    ON tl_id = tln_lang
        AND tln_name_lang = language_of_user

    WHERE tpt_post = post_id

    ORDER BY tln_name
)

$$;
