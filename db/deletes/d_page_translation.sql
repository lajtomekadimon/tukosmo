
CREATE OR REPLACE FUNCTION d_page_translation(
    page_trans_id BIGINT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_page_translations
    WHERE tpt_id = page_trans_id;

    RETURN page_trans_id;

END;

$$;
