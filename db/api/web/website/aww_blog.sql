
CREATE OR REPLACE FUNCTION aww_blog(
    language_of_user INT8
)

RETURNS "PostDB"[]

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT s_posts_by_lang(language_of_user)

$$;
