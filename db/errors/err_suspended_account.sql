
CREATE OR REPLACE FUNCTION err_suspended_account()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:76417C47169A';

END;

$$;