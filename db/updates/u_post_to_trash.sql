
CREATE OR REPLACE FUNCTION u_post_to_trash(
    post_id BIGINT
)

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_post_translations
    SET tpt_deleted = TRUE
    WHERE tpt_post = post_id;

END;

$$;
