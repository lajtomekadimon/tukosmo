
CREATE OR REPLACE FUNCTION u_language(
    lang_id BIGINT,
    code_value TEXT
)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_languages
    SET tl_code = code_value
    WHERE tl_id = lang_id;

    RETURN lang_id;

END;

$$;
