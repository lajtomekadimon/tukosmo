
CREATE OR REPLACE FUNCTION d_tag_translation(
    tag_trans_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_tag_translations
    WHERE ttt_id = tag_trans_id;

    RETURN tag_trans_id;

END;

$$;

