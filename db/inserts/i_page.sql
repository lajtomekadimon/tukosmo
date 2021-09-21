
CREATE OR REPLACE FUNCTION i_page(
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

    INSERT INTO t_pages (
        --tp_id,
        tp_author
        --tp_date
    )
    VALUES (
        -- BIGSERIAL (autoincrement)
        author_id
        --NOW()
    )
    RETURNING tp_id INTO page_id;

    RETURN page_id;

END;

$$;
