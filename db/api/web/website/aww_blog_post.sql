
CREATE OR REPLACE FUNCTION aww_blog_post(
    language_of_user INT8,
    permalink_value TEXT
)

RETURNS "PostDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT (
    id,
    trans_id,
    lang,
    title,
    description,
    body,
    permalink,
    author,
    author_name,
    translator,
    translator_name,
    date,
    date_trans,
    draft,
    deleted
)::"PostDB"

FROM s_post_by_lang_permalink(
    language_of_user,
    permalink_value
)

$$;
