
CREATE OR REPLACE FUNCTION s_post_by_id_lang(
    post_id BIGINT,
    post_lang BIGINT
)

RETURNS "PostDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    -- id
    tp_id,
    -- featured_image
    s_file_by_id(
        tp_featured_image,
        post_lang
    ),
    -- trans_id
    tpt_id,
    -- lang
    s_language_by_id_lang(
        tpt_lang,
        post_lang
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
    COALESCE(
        s_user_name_by_user_lang(
            b.tu_id,
            post_lang
        ),
        b.tu_name
    ),
    -- translator
    tpt_translator,
    -- translator_name
    COALESCE(
        s_user_name_by_user_lang(
            a.tu_id,
            post_lang
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
)::"PostDB"

FROM t_posts

INNER JOIN t_post_translations
ON tp_id = tpt_post
    AND tpt_lang = post_lang
    AND tp_id = post_id

INNER JOIN t_users a
ON tpt_translator = a.tu_id

INNER JOIN t_users b
ON tp_author = b.tu_id

ORDER BY tp_date DESC
LIMIT 1

$$;
