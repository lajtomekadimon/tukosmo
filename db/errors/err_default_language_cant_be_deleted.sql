
CREATE OR REPLACE FUNCTION err_default_language_cant_be_deleted()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:6499545DEC11';

END;

$$;
