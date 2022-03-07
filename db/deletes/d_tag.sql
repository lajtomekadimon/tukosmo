
CREATE OR REPLACE FUNCTION d_tag(
    tag_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_tags
    WHERE tt_id = tag_id;

    RETURN tag_id;

END;

$$;