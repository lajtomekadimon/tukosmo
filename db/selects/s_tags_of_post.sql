
CREATE OR REPLACE FUNCTION s_tags_of_post(
    language_of_user BIGINT,
    post_id BIGINT
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
        COALESCE(ttt_id, 0),

        -- lang
        COALESCE(
            s_language_by_id_lang(
                ttt_lang,
                language_of_user
            ),
            (
                0, '', '', '', '', FALSE
            )::"LanguageDB"
        ),

        -- name
        COALESCE(
            ttt_name,
            s_untrans_tag_name_by_id(tt_id)
        ),

        -- permalink
        COALESCE(ttt_permalink, ''),

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
        COALESCE(ttt_translator, 0),

        -- translator_name
        COALESCE(
            s_user_name_by_user_lang(
                a.tu_id,
                language_of_user
            ),
            a.tu_name,
            ''
        ),

        -- date
        tt_date::TEXT,

        -- date_trans
        COALESCE(ttt_date::TEXT, '')
    )::"TagDB"

    FROM t_tags_of_posts

    INNER JOIN t_tags
    ON ttop_tag = tt_id

    LEFT JOIN t_tag_translations
    ON tt_id = ttt_tag
        AND CASE
            WHEN ttt_lang IS NULL
            THEN TRUE
            ELSE ttt_lang = language_of_user
        END

    LEFT JOIN t_users a
    ON ttt_translator = a.tu_id

    INNER JOIN t_users b
    ON tt_author = b.tu_id

    ORDER BY ttt_name DESC
)

$$;
