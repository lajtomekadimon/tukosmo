
CREATE OR REPLACE FUNCTION err_wrong_lang_code()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:9F4297D0460F';

END;

$$;
