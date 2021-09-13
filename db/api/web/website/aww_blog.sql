
CREATE OR REPLACE FUNCTION aww_blog(
    language_of_user INT8
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

SELECT s_posts_by_lang(language_of_user)

$$;
