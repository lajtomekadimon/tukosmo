
CREATE OR REPLACE FUNCTION s_post_fimage_by_id_lang(
    post_id BIGINT,
    language_of_user BIGINT
)

RETURNS "FileDB"

LANGUAGE SQL
VOLATILE
RETURNS NULL ON NULL INPUT
PARALLEL UNSAFE

AS $$

SELECT s_file_by_id(
    tp_featured_image,
    language_of_user
)
FROM t_posts

WHERE tp_id = post_id

LIMIT 1

$$;
