
CREATE OR REPLACE FUNCTION aww_blog_post(
    language_of_user INT8,
    permalink_value TEXT
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
    date_trans TEXT
)

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT s_post_by_lang_permalink(
    language_of_user,
    permalink_value
)

$$;
