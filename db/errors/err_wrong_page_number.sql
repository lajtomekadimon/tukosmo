
CREATE OR REPLACE FUNCTION err_wrong_page_number()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:58CC6F302A91';

END;

$$;
