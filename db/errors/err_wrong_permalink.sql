
CREATE OR REPLACE FUNCTION err_wrong_permalink()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:7D637808B8AA';

END;

$$;
