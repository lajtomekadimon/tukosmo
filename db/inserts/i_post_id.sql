
CREATE OR REPLACE FUNCTION i_post_id(
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

    INSERT INTO t_post_ids (
        --tpi_id,
        tpi_author
        --tpi_date
    )
    VALUES (
        -- BIGSERIAL (autoincrement)
        author_id
        --NOW()
    )
    RETURNING tpi_id INTO post_id;

    RETURN post_id;

END;

$$;
