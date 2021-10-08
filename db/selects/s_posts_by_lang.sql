
CREATE OR REPLACE FUNCTION s_posts_by_lang(
    language_of_user BIGINT
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
        tpt_lang,
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
        tp_date,
        -- date_trans
        tpt_date,
        -- draft
        tpt_draft,
        -- deleted
        tpt_deleted
    )::"PostDB"

    FROM t_posts

    INNER JOIN t_post_translations
    ON tp_id = tpt_post
        AND tpt_lang = language_of_user
        AND (NOT tpt_deleted)
        AND (NOT tpt_draft)

    INNER JOIN t_users a
    ON tpt_translator = a.tu_id

    INNER JOIN t_users b
    ON tp_author = b.tu_id

    ORDER BY tp_date DESC
)

$$;
