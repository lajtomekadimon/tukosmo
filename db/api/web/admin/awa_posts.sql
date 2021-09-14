
CREATE OR REPLACE FUNCTION awa_posts(
    language_of_user BIGINT
)

RETURNS TABLE(
    id BIGINT,
    trans_id BIGINT,
    lang BIGINT,
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

SELECT s_posts(language_of_user)

$$;
