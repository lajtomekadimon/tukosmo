
CREATE OR REPLACE FUNCTION i_language_name(

    lang_id BIGINT,

    name_value TEXT,

    name_lang_id BIGINT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

DECLARE

    lang_name_id BIGINT;

BEGIN

    INSERT INTO t_language_names (
        --tln_id,
        tln_lang,
        tln_name,
        tln_name_lang
        --tln_date
    ) VALUES (
        -- BIGSERIAL (autoincrement)
        lang_id,
        name_value,
        name_lang_id
        --NOW()
    ) RETURNING tln_id INTO lang_name_id;

    RETURN lang_name_id;

END;

$$;
