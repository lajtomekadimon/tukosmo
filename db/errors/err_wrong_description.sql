
CREATE OR REPLACE FUNCTION err_wrong_description()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:4E576E6BB1EE';

END;

$$;
