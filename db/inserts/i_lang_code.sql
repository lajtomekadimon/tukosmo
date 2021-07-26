
CREATE OR REPLACE FUNCTION i_lang_code(

    code_value TEXT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    lang_code_id BIGINT;

BEGIN

    INSERT INTO t_lang_codes (
        --tlc_id,
        tlc_code
        --tlc_date
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        code_value
        --NOW()
    ) RETURNING tlc_id INTO lang_code_id;

    RETURN lang_code_id;

END;

$$;
