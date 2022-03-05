
CREATE OR REPLACE FUNCTION d_tags_of_post(
    post_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_tags_of_posts
    WHERE ttop_post = post_id;

    RETURN post_id;

END;

$$;

