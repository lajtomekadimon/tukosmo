
CREATE OR REPLACE FUNCTION s_posts_by_pref_lang(
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
    SELECT x.post FROM (
        SELECT DISTINCT ON (tp_id) (
            -- id
            tp_id,
            -- featured_image
            s_file_by_id(
                tp_featured_image,
                language_of_user
            ),
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
            -- tags
            s_tags_of_post(language_of_user, tp_id),
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
            tpt_translator,
            -- translator_name
            COALESCE(
                s_user_name_by_user_lang(
                    a.tu_id,
                    language_of_user
                ),
                a.tu_name
            ),
            -- date
            tp_date,
            -- date_trans
            tpt_date,
            -- draft
            tpt_draft,
            -- deleted
            tpt_deleted
        )::"PostDB" AS post,

        tp_date AS tp_date

        FROM t_posts

        INNER JOIN t_post_translations
        ON tp_id = tpt_post
            --AND tpt_lang = language_of_user
            AND (NOT tpt_deleted)
            AND (NOT tpt_draft)

        INNER JOIN t_users a
        ON tpt_translator = a.tu_id

        INNER JOIN t_users b
        ON tp_author = b.tu_id

        ORDER BY tp_id, CASE
            WHEN tpt_lang = language_of_user THEN 1
            -- TODO: Add English between 1 and 2
            ELSE 2
        END ASC
    ) x
    ORDER BY x.tp_date DESC

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
