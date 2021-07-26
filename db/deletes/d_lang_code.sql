
CREATE OR REPLACE FUNCTION d_lang_code(

    lang_code_id BIGINT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    DELETE FROM t_lang_codes
    WHERE tlc_id = lang_code_id;

    RETURN lang_code_id;

END;

$$;
