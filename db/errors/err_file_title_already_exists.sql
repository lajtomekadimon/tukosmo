
CREATE OR REPLACE FUNCTION err_file_title_already_exists()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:09DD33C2C469';

END;

$$;
