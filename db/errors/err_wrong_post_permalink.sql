
CREATE OR REPLACE FUNCTION err_wrong_post_permalink()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:C90EAC6F8E2E';

END;

$$;
