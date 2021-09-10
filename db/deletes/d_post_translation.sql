
CREATE OR REPLACE FUNCTION d_post_translation(

    post_trans_id BIGINT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_post_translations
    WHERE tpt_id = post_trans_id;

    RETURN post_trans_id;

END;

$$;

