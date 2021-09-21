
CREATE OR REPLACE FUNCTION awa_edit_post(
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

FROM s_post_by_id_lang(
    post_id,
    post_lang
)

$$;
