
CREATE OR REPLACE FUNCTION u_post(
    post_id BIGINT,
    featured_image_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    post_id BIGINT;

BEGIN

    UPDATE t_posts
    SET tp_featured_image = featured_image_id
    WHERE tp_id = post_id
    RETURNING tp_id INTO post_id;


    RETURN post_id;

END;

$$;
