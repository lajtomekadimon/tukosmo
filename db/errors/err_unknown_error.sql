
CREATE OR REPLACE FUNCTION err_unknown_error()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:BCDD2EA230A5';

END;

$$;
