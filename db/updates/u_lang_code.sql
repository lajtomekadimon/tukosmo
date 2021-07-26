
CREATE OR REPLACE FUNCTION u_lang_code(

    lang_code_id BIGINT,

    code_value TEXT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_lang_codes
    SET tlc_code = code_value
    WHERE tlc_id = lang_code_id;

    RETURN lang_code_id;

END;

$$;
