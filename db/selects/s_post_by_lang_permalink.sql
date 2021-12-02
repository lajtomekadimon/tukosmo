
CREATE OR REPLACE FUNCTION s_post_by_lang_permalink(
    language_of_user BIGINT,
    permalink_value TEXT
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
)::"PostDB"

FROM t_posts

INNER JOIN t_post_translations
ON tp_id = tpt_post
    AND tpt_lang = language_of_user
    AND tpt_permalink = permalink_value
    AND (NOT tpt_deleted)
    AND (NOT tpt_draft)

INNER JOIN t_users a
ON tpt_translator = a.tu_id

INNER JOIN t_users b
ON tp_author = b.tu_id

ORDER BY tp_date DESC
LIMIT 1

$$;
