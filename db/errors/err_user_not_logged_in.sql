
CREATE OR REPLACE FUNCTION err_user_not_logged_in()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:D994FEF2356A';

END;

$$;
