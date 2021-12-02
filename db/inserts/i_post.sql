
CREATE OR REPLACE FUNCTION i_post(
    featured_image_id BIGINT,
    author_id BIGINT
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

    INSERT INTO t_posts (
        --tp_id,
        tp_featured_image,
        tp_author
        --tp_date
    )
    VALUES (
        -- BIGSERIAL (autoincrement)
        featured_image_id,
        author_id
        --NOW()
    )
    RETURNING tp_id INTO post_id;

    RETURN post_id;

END;

$$;
