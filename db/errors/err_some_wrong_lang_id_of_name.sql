
CREATE OR REPLACE FUNCTION err_some_wrong_lang_id_of_name()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:55F1A77CE041';

END;

$$;
