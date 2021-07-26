
CREATE OR REPLACE FUNCTION i_language(

    lang_code_id BIGINT,

    name_value TEXT,

    lang_id BIGINT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    language_id BIGINT;

BEGIN

    INSERT INTO t_languages (
        --tl_id,
        tl_lang_code,
        tl_name,
        tl_lang
        --tl_date
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        lang_code_id,
        name_value,
        lang_id
        --NOW()
    ) RETURNING tl_id INTO language_id;

    RETURN language_id;

END;

$$;
