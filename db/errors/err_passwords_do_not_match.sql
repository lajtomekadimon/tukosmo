
CREATE OR REPLACE FUNCTION err_passwords_do_not_match()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:51FE0497F895';

END;

$$;
