
CREATE OR REPLACE FUNCTION s_posts_f_untranslated(
    language_of_user BIGINT,
    results_number BIGINT,
    page_number BIGINT
)

RETURNS "PostDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ARRAY(
    SELECT (
        -- id
        tp_id,

        -- trans_id
        0,

        -- lang
        (
            0, '', '', '', '', FALSE
        )::"LanguageDB",

        -- title
        s_untrans_post_title_by_id(tp_id),

        -- description
        '',

        -- body
        '',

        -- permalink
        '',

        -- author
        tp_author,

        -- author_name
        COALESCE(
            s_user_name_by_user_lang(
                b.tu_id,
                language_of_user
            ),
            b.tu_name
        ),

        -- translator
        0,

        -- translator_name
        '',

        -- date
        tp_date::TEXT,

        -- date_trans
        '',

        -- draft
        TRUE,

        -- deleted
        FALSE
    )::"PostDB"

    FROM t_posts

    LEFT JOIN t_post_translations
    ON tp_id = tpt_post
        AND tpt_lang = language_of_user

    LEFT JOIN t_users a
    ON tpt_translator = a.tu_id

    INNER JOIN t_users b
    ON tp_author = b.tu_id

    WHERE tpt_post IS NULL

    ORDER BY tp_date DESC

    /* IMPORTANT NOTE:
     * Pagination using LIMIT + OFFSET is not an efficient solution, as it gets
     * slower and slower as the table grows. Indexing immutable rows could be
     * a solution, but then you'd have pages with different number of results
     * once you delete something.
     */
    LIMIT results_number
    OFFSET (page_number - 1) * results_number
)

$$;
