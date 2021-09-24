
CREATE OR REPLACE FUNCTION s_posts(
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
        COALESCE(tpt_id, 0),

        -- lang
        COALESCE(tpt_lang, 0),

        -- title
        COALESCE(
            tpt_title,
            s_untrans_post_title_by_id(tp_id)
        ),

        -- description
        COALESCE(tpt_description, ''),

        -- body
        COALESCE(tpt_body, ''),

        -- permalink
        COALESCE(tpt_permalink, ''),

        -- author
        tp_author,

        -- author_name
        b.tu_name,

        -- translator
        COALESCE(tpt_translator, 0),

        -- translator_name
        COALESCE(a.tu_name, ''),

        -- date
        tp_date::TEXT,

        -- date_trans
        COALESCE(tpt_date::TEXT, ''),

        -- draft
        COALESCE(tpt_draft, TRUE),

        -- deleted
        COALESCE(tpt_deleted, FALSE)
    )::"PostDB"

    FROM t_posts

    LEFT JOIN t_post_translations
    ON tp_id = tpt_post
        AND CASE
            WHEN tpt_lang IS NULL
            THEN TRUE
            ELSE tpt_lang = language_of_user
        END
        AND NOT tpt_deleted

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
