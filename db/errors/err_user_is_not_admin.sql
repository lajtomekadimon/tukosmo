
CREATE OR REPLACE FUNCTION err_user_is_not_admin()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:A63801E4322A';

END;

$$;
