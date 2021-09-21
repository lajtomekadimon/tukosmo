
CREATE OR REPLACE FUNCTION aww_blog(
    language_of_user INT8
)

RETURNS "PostDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT ARRAY(
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

    FROM s_posts_by_lang(language_of_user)
)

$$;
