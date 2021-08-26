
CREATE OR REPLACE FUNCTION u_language_name(

    lang_name_id BIGINT,

    name_value TEXT

)

RETURNS BIGINT

LANGUAGE PLPGSQL
VOLATILE
CALLED ON NULL INPUT
PARALLEL UNSAFE

AS $$

BEGIN

    UPDATE t_language_names
    SET tln_name = name_value
    WHERE tln_id = lang_name_id;

    RETURN lang_name_id;

END;

$$;
