
CREATE OR REPLACE FUNCTION i_language(
    code_value TEXT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    lang_id BIGINT;

BEGIN

    INSERT INTO t_languages (
        --tl_id,
        tl_code
        --tl_date
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        code_value
        --NOW()
    ) RETURNING tl_id INTO lang_id;

    RETURN lang_id;

END;

$$;
