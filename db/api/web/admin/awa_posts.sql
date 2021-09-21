
CREATE OR REPLACE FUNCTION awa_posts(
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

    FROM s_posts(language_of_user)
)


$$;
