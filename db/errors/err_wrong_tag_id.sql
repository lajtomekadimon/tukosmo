
CREATE OR REPLACE FUNCTION err_wrong_tag_id()

RETURNS VOID

LANGUAGE PLPGSQL
VOLATILE
PARALLEL UNSAFE

AS $$

BEGIN

    RAISE EXCEPTION 'TUKOSMO:406517953D5E';

END;

$$;
