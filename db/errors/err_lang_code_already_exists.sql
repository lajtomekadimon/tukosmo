
CREATE OR REPLACE FUNCTION err_lang_code_already_exists()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:4C66AB9F871B';

END;

$$;
