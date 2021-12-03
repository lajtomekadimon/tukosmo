
CREATE OR REPLACE FUNCTION u_posts_del_images(
    featured_image_id BIGINT
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_posts
    SET tp_featured_image = NULL
    WHERE tp_featured_image = featured_image_id;

END;

$$;
