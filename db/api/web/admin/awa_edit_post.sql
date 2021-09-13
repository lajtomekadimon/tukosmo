
CREATE OR REPLACE FUNCTION awa_edit_post(
    post_id BIGINT,
    post_lang BIGINT
)

RETURNS TABLE(
    id BIGINT,
    title TEXT,
    description TEXT,
    body TEXT,
    permalink TEXT,
    author BIGINT,
    author_name TEXT,
    translator BIGINT,
    translator_name TEXT,
    date TEXT,
    date_trans TEXT,
    draft BOOL,
    deleted BOOL
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT s_post_by_id_lang(
    post_id,
    post_lang
)

$$;
