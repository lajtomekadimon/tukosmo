
CREATE OR REPLACE FUNCTION i_tag(
    author_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    tag_id BIGINT;

BEGIN

    INSERT INTO t_tags (
        --tt_id,
        --tt_date,
        tt_author
    )
    VALUES (
        -- BIGSERIAL (autoincrement)
        --NOW(),
        author_id
    )
    RETURNING tt_id INTO tag_id;

    RETURN tag_id;

END;

$$;
