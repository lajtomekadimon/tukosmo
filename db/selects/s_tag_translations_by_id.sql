
CREATE OR REPLACE FUNCTION s_tag_translations_by_id(
    language_of_user BIGINT,
    tag_id BIGINT
)

RETURNS "TagDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ARRAY(
    SELECT (
        -- id
        tt_id,

        -- trans_id
        ttt_id,

        -- lang
        s_language_by_id_lang(
            ttt_lang,
            language_of_user
        ),
            
        -- name
        ttt_name,

        -- permalink
        ttt_permalink,

        -- author
        tt_author,

        -- author_name
        COALESCE(
            s_user_name_by_user_lang(
                b.tu_id,
                language_of_user
            ),
            b.tu_name
        ),

        -- translator
        ttt_translator,

        -- translator_name
        COALESCE(
            s_user_name_by_user_lang(
                a.tu_id,
                language_of_user
            ),
            a.tu_name
        ),

        -- date
        tt_date::TEXT,

        -- date_trans
        ttt_date::TEXT
    )::"TagDB"

    FROM t_tags

    LEFT JOIN t_tag_translations
    ON tt_id = ttt_tag

    LEFT JOIN t_users a
    ON ttt_translator = a.tu_id

    INNER JOIN t_users b
    ON tt_author = b.tu_id

    WHERE tt_id = tag_id

    ORDER BY ttt_lang DESC
)

$$;
