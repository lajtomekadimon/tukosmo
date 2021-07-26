
CREATE OR REPLACE FUNCTION i_post_id()

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    post_id BIGINT;

BEGIN

    INSERT INTO t_post_ids
    DEFAULT VALUES
        --tpi_id
        --tpi_date
    RETURNING tpi_id INTO post_id;

    RETURN post_id;

END;

$$;
