
CREATE OR REPLACE FUNCTION i_page_id(
    author_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    page_id BIGINT;

BEGIN

    INSERT INTO t_page_ids (
        --tpi_id,
        tpi_author
        --tpi_date
    )
    VALUES (
        -- BIGSERIAL (autoincrement)
        author_id
        --NOW()
    )
    RETURNING tpi_id INTO page_id;

    RETURN page_id;

END;

$$;
