
CREATE OR REPLACE FUNCTION err_wrong_website_subtitle()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:CF625B956EBD';

END;

$$;
