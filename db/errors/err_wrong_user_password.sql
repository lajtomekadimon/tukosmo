
CREATE OR REPLACE FUNCTION err_wrong_user_password()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:C37F7C062377';

END;

$$;
