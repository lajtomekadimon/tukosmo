
CREATE OR REPLACE FUNCTION i_post(
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
        tp_author
        --tp_date
    )
    VALUES (
        -- BIGSERIAL (autoincrement)
        author_id
        --NOW()
    )
    RETURNING tp_id INTO post_id;

    RETURN post_id;

END;

$$;
