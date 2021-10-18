
CREATE OR REPLACE FUNCTION s_posts_f_published(
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
        tpt_id,

        -- lang
        s_language_by_id_lang(
            tpt_lang,
            language_of_user
        ),

        -- title
        tpt_title,

        -- description
        tpt_description,

        -- body
        tpt_body,

        -- permalink
        tpt_permalink,

        -- author
        tp_author,

        -- author_name
        b.tu_name,

        -- translator
        tpt_translator,

        -- translator_name
        a.tu_name,

        -- date
        tp_date::TEXT,

        -- date_trans
        tpt_date::TEXT,

        -- draft
        tpt_draft,

        -- deleted
        tpt_deleted
    )::"PostDB"

    FROM t_posts

    INNER JOIN t_post_translations
    ON tp_id = tpt_post
        AND tpt_lang = language_of_user
        AND (NOT tpt_draft)
        AND (NOT tpt_deleted)

    LEFT JOIN t_users a
    ON tpt_translator = a.tu_id

    INNER JOIN t_users b
    ON tp_author = b.tu_id

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
