
CREATE OR REPLACE FUNCTION i_page_id()

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    page_id BIGINT;

BEGIN

    INSERT INTO t_page_ids
    DEFAULT VALUES
        --tpi_id
        --tpi_date
    RETURNING tpi_id INTO page_id;

    RETURN page_id;

END;

$$;
